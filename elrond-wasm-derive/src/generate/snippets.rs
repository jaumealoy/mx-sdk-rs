pub fn contract_object_def() -> proc_macro2::TokenStream {
    quote! {
        pub struct ContractObj<A>
        where
            A: elrond_wasm::api::VMApi,
        {
            api: A,
        }
    }
}

pub fn impl_contract_base() -> proc_macro2::TokenStream {
    quote! {
        impl<A> elrond_wasm::contract_base::ContractBase for ContractObj<A>
        where
            A: elrond_wasm::api::VMApi,
        {
            type Api = A;

            fn raw_vm_api(&self) -> Self::Api {
                self.api.clone()
            }
        }
    }
}

pub fn new_contract_object_fn() -> proc_macro2::TokenStream {
    quote! {
        pub fn contract_obj<A>(api: A) -> ContractObj<A>
        where
            A: elrond_wasm::api::VMApi,
        {
            ContractObj { api }
        }
    }
}

// TODO: explore auto-implementations of supertraits
#[allow(dead_code)]
pub fn impl_auto_impl() -> proc_macro2::TokenStream {
    quote! {
        impl<A> AutoImpl for ContractObj<A> where
            A: elrond_wasm::contract_base::ContractBase
                + elrond_wasm::api::ErrorApi
                + elrond_wasm::api::EndpointArgumentApi
                + elrond_wasm::api::EndpointFinishApi
                + elrond_wasm::api::ManagedTypeApi
        {
        }
    }
}

pub fn impl_callable_contract() -> proc_macro2::TokenStream {
    quote! {
        impl<A> elrond_wasm::contract_base::CallableContract<A> for ContractObj<A>
        where
            A: elrond_wasm::api::VMApi,
        {
            fn call(&self, fn_name: &[u8]) -> bool {
                EndpointWrappers::call(self, fn_name)
            }
            fn into_api(self: Box<Self>) -> A {
                self.api
            }
        }
    }
}

pub fn proxy_object_def() -> proc_macro2::TokenStream {
    quote! {
        pub struct Proxy<A>
        where
            A: elrond_wasm::api::VMApi + 'static,
        {
            pub address: elrond_wasm::types::ManagedAddress<A>,
        }

        impl<A> elrond_wasm::contract_base::ProxyObjBase for Proxy<A>
        where
            A: elrond_wasm::api::VMApi + 'static,
        {
            type Api = A;

            fn new_proxy_obj() -> Self {
                let zero_address = ManagedAddress::zero();
                Proxy {
                    address: zero_address,
                }
            }

            fn contract(mut self, address: ManagedAddress<Self::Api>) -> Self {
                self.address = address;
                self
            }

            #[inline]
            fn into_fields(self) -> ManagedAddress<Self::Api> {
                self.address
            }
        }
    }
}

pub fn callback_proxy_object_def() -> proc_macro2::TokenStream {
    quote! {
        pub struct CallbackProxyObj<A>
        where
            A: elrond_wasm::api::VMApi + 'static,
        {
            _phantom: core::marker::PhantomData<A>,
        }

        impl<A> elrond_wasm::contract_base::CallbackProxyObjBase for CallbackProxyObj<A>
        where
            A: elrond_wasm::api::VMApi + 'static,
        {
            type Api = A;

            fn new_cb_proxy_obj() -> Self {
                CallbackProxyObj {
                    _phantom: core::marker::PhantomData,
                }
            }
        }
    }
}
