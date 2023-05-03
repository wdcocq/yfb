use darling::{ast::Data, FromDeriveInput, FromField, FromMeta, ToTokens};
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Ident, Path, Type, Visibility};

#[proc_macro_derive(Model, attributes(yfb))]
pub fn derive_model(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    match ModelReceiver::from_derive_input(&ast) {
        Ok(receiver) => receiver.to_token_stream(),
        Err(e) => e.write_errors(),
    }
    .into()
}
#[derive(Debug, FromField)]
struct ModelField {
    ident: Option<Ident>,
    vis: Visibility,
    ty: Type,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(yfb), supports(struct_named))]
struct ModelReceiver {
    ident: Ident,
    vis: Visibility,
    data: Data<(), ModelField>,
    #[darling(default)]
    path: CratePath,
}

#[derive(Debug, FromMeta)]
struct CratePath(Path);

impl Default for CratePath {
    fn default() -> Self {
        Self(syn::parse_str("::yfb").unwrap())
    }
}

impl ToTokens for CratePath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}

impl ToTokens for ModelReceiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let yfb = &self.path;
        let fields = self
            .data
            .as_ref()
            .take_struct()
            .expect("only named structs are supported")
            .fields;

        let (state, state_ident) = self.expand_state(&fields);
        let (mappings, mapping_idents) = self.expand_mappings(&fields);
        let (modifier, modifier_ident) = self.expand_modifier(&fields, &mapping_idents);
        let binding_ext = self.expand_binding_ext(&fields, &mapping_idents);
        let model_name = ident.to_string().to_snake_case();

        tokens.extend(quote! {
            impl #yfb::model::Model for #ident {
                const NAME: &'static str = #model_name;
            }

            impl #yfb::model::ModelState for #ident {
                type State = #state_ident;
                type Modifier = #modifier_ident;
            }

            #state
            #modifier
            #(#mappings)*
            #binding_ext
        });
    }
}

impl ModelReceiver {
    fn expand_state(&self, fields: &[&ModelField]) -> (TokenStream, Ident) {
        let yfb = &self.path;
        let vis = &self.vis;
        let model_ident = &self.ident;
        let state_ident = format_ident!("{}State", model_ident);

        let (state_fields, field_idents) = fields
            .iter()
            .map(|f| {
                let ident = f.ident.as_ref().unwrap();
                let ty = &f.ty;
                let vis = &f.vis;
                (
                    quote! {
                        #vis #ident: <#ty as #yfb::model::ModelState>::State
                    },
                    ident,
                )
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();

        (
            quote! {
                #[derive(Debug, PartialEq)]
                #vis struct #state_ident {
                    #(#state_fields),*
                }

                impl #yfb::model::State<#model_ident> for #state_ident {
                    fn create(model: &#model_ident, with_initial: ::std::primitive::bool) -> Self {
                        Self {
                            #(#field_idents: #yfb::model::State::create(&model.#field_idents, with_initial)),*
                        }
                    }

                    fn update(&mut self, model: &#model_ident) {
                        #(
                            self.#field_idents.update(&model.#field_idents);
                        )*
                    }
                }

                impl #yfb::model::Dirty for #state_ident {
                    fn dirty(&self) -> ::std::primitive::bool {
                        false
                        #(
                            | #yfb::model::Dirty::dirty(&self.#field_idents)
                        )*
                    }
                }
            },
            state_ident,
        )
    }

    fn expand_mappings(&self, fields: &[&ModelField]) -> (Vec<TokenStream>, Vec<Ident>) {
        let yfb = &self.path;
        let model_ident = &self.ident;

        fields
            .iter()
            .map(|f| {
                let ident = f.ident.as_ref().unwrap();
                let mapping_ident = format_ident!(
                    "{}{}Mapping",
                    model_ident,
                    ident.to_string().to_pascal_case()
                );
                let ident_name = ident.to_string();
                let ty = &f.ty;
                let vis = &f.vis;
                (
                    quote! {
                        #vis struct #mapping_ident;

                        impl #yfb::state_model::Mapping for #mapping_ident {
                            const NAME: &'static ::std::primitive::str = #ident_name;
                            type From = #model_ident;
                            type To = #ty;

                            fn map_model<'a>(&self, model: &'a Self::From) -> &'a Self::To {
                                &model.#ident
                            }

                            fn map_state<'a>(
                                &self,
                                state: &'a <Self::From as #yfb::model::ModelState>::State,
                            ) -> &'a <Self::To as #yfb::model::ModelState>::State {
                                &state.#ident
                            }

                            fn map_model_mut<'a>(&self, model: &'a mut Self::From) -> &'a mut Self::To {
                                &mut model.#ident
                            }

                            fn map_state_mut<'a>(
                                &self,
                                state: &'a mut <Self::From as #yfb::model::ModelState>::State,
                            ) -> &'a mut <Self::To as #yfb::model::ModelState>::State {
                                &mut state.#ident
                            }
                        }
                    },
                    mapping_ident,
                )
            })
            .unzip()
    }

    fn expand_modifier(&self, fields: &[&ModelField], mappings: &[Ident]) -> (TokenStream, Ident) {
        let yfb = &self.path;
        let model_ident = &self.ident;
        let vis = &self.vis;
        let modifier_ident = format_ident!("{}Modifier", model_ident);

        let (field_modifiers, field_set_message) = fields
            .iter()
            .zip(mappings)
            .map(|(f, m)| {
                let ident = f.ident.as_ref().unwrap();
                let ident_name = format!("{}", ident);
                let ty = &f.ty;
                (
                    quote! {
                        fn #ident(&self) -> <#ty as #yfb::model::ModelState>::Modifier {
                            #yfb::modifier::Modifier::map(&self.0, #m)
                        }
                    },
                    quote! {
                        #ident_name => {
                            let modifier = self.#ident();
                            if modifier.dirty() {
                                if let Some(error) = errors.first() {
                                    modifier.set_message(error.message.clone().map(|m| m.into()));
                                }
                            }
                        },
                    },
                )
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();

        let field_idents = fields
            .iter()
            .map(|f| f.ident.as_ref().unwrap())
            .collect::<Vec<_>>();

        (
            quote! {
                #vis struct #modifier_ident(#yfb::modifier::BaseModifier<#model_ident>);

                impl #yfb::modifier::Modifier<#model_ident> for #modifier_ident {
                    fn create(
                        provider: #yfb::state_model::StateModelRc<#model_ident>,
                        validation: ::std::rc::Rc<#yfb::binding::BindingValidation>) -> Self {
                        Self(#yfb::modifier::Modifier::create(provider, validation))
                    }

                    fn state_model(&self) -> &#yfb::state_model::StateModelRc<#model_ident> {
                        #yfb::modifier::Modifier::state_model(&self.0)
                    }

                    fn validation(&self) -> &::std::rc::Rc<#yfb::binding::BindingValidation> {
                        #yfb::modifier::Modifier::validation(&self.0)
                    }
                }

                impl #modifier_ident {
                    #(#field_modifiers)*
                }

                impl Drop for #modifier_ident  {
                    fn drop(&mut self) {
                        if #yfb::modifier::Modifier::validation(self).valid() {
                            return;
                        }

                        if !#yfb::modifier::Modifier::dirty(self) {
                            return;
                        }

                        let validation = {
                            let state_model = #yfb::modifier::Modifier::state_model(&self.0);
                            ::validator::Validate::validate(&*state_model.model())
                        };

                        #(
                            self.#field_idents().set_message(None);
                        )*

                        let Err(errors) = validation else {
                            return;
                        };

                        for (field, errors) in errors.field_errors() {
                            match field {
                                #(#field_set_message)*
                                _ => {}
                            }
                        }
                    }
                }
            },
            modifier_ident,
        )
    }

    fn expand_binding_ext(&self, fields: &[&ModelField], mappings: &[Ident]) -> TokenStream {
        let yfb = &self.path;
        let model_ident = &self.ident;
        let trait_ident = format_ident!("{}BindingExt", self.ident);
        let vis = &self.vis;
        let (binding_fn_defs, binding_fns) = fields
            .iter()
            .zip(mappings)
            .map(|(f, m)| {
                let fn_ident = format_ident!("{}_binding", &f.ident.as_ref().unwrap());
                let ty = &f.ty;

                (
                    quote! {
                        fn #fn_ident(&self) -> #yfb::binding::Binding<#ty>;
                    },
                    quote! {
                        fn #fn_ident(&self) -> #yfb::binding::Binding<#ty> {
                            #yfb::binding::Binding::map(self, #m)
                        }
                    },
                )
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();

        quote! {
            #vis trait #trait_ident {
                #(#binding_fn_defs)*
            }

            impl #trait_ident for #yfb::binding::Binding<#model_ident> {
                #(#binding_fns)*
            }
        }
    }
}
