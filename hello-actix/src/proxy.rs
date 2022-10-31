use reqwest::{Client, Proxy};

pub struct ProxyClientProps {
    pub(crate) host: Option<String>,
    pub(crate) port: Option<u16>,
    pub(crate) username: Option<String>,
    pub(crate) password: Option<String>,
}

pub fn get_proxy_client(props: ProxyClientProps) -> Client {
    if props.host.is_none() || props.port.is_none() {
        return Client::new();
    }

    if props.username.is_none() || props.password.is_none() {
        return Client::builder()
            .proxy(Proxy::http(props.host.unwrap()).unwrap())
            .build()
            .unwrap();
    }

    return Client::builder()
        .proxy(
            Proxy::http(props.host.unwrap())
                .unwrap()
                .basic_auth(&props.username.unwrap(), &props.password.unwrap()),
        )
        .build()
        .unwrap();
}
