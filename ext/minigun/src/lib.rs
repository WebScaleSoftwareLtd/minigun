use std::sync::OnceLock;
use magnus::{
    define_module, RHash, Value, Object, function,
    RString, Ruby, Error, RClass, value::ReprValue, Symbol,
    RArray,
};

// Share a HTTP client across all threads.
static CLIENT: OnceLock<reqwest::blocking::Client> = OnceLock::new();

fn do_request (
    url: RString,
    obj: Option<RHash>,
    method: reqwest::Method,
) -> Result<RHash, Error> {
    // Get the HTTP client.
    let client = CLIENT.get().unwrap();
    let ruby_handle = Ruby::get_with(url);

    // Figure out if the URL is a string or a URI object.
    let url_s = url.to_string().unwrap();

    // Try and parse the URL.
    let url = match reqwest::Url::parse(&url_s) {
        Ok(url) => url,
        Err(err) => {
            return Err(Error::new(
                ruby_handle.exception_runtime_error(),
                format!("failed to parse URL: {}", err),
            ));
        }
    };

    // Make a request builder.
    let mut builder = client.request(method, url);
    let mut read_body = true;
    if obj.is_some() {
        let obj = obj.unwrap();

        // Get the headers.
        let headers = obj.get(Symbol::new("headers"));
        if headers.is_some() {
            // Get the headers.
            let headers = RHash::from_value(headers.unwrap());
            if headers.is_some() {
                let headers = headers.unwrap();

                // Iterate over the headers.
                for item in headers.enumeratorize("each", ()) {
                    // If there was a error, return it.
                    if item.is_err() {
                        return Err(item.unwrap_err());
                    }

                    // Get the item.
                    let item = item.unwrap();

                    // Turn it into an array.
                    let item = RArray::from_value(item);
                    if item.is_none() {
                        continue;
                    }

                    // Get the key and value.
                    let item = item.unwrap();
                    let key = item.entry::<RString>(0);
                    let value = item.entry::<RString>(1);

                    // If there was a error, return it.
                    if key.is_err() {
                        return Err(key.unwrap_err());
                    }
                    if value.is_err() {
                        return Err(value.unwrap_err());
                    }

                    // Unwrap the value.
                    let key = key.unwrap();
                    let value = value.unwrap();

                    // Set the header.
                    builder = builder.header(key.to_string().unwrap(), value.to_string().unwrap());
                }
            }
        }

        // Get the body.
        let body = obj.get(Symbol::new("body"));
        if body.is_some() {
            // Get the body.
            let body = RString::from_value(body.unwrap());
            if body.is_some() {
                let body = body.unwrap();

                // Set the body.
                // TODO: Handle non-UTF8 bodies.
                builder = builder.body(body.to_string().unwrap());
            } else {
                // Throw a type error.
                return Err(Error::new(
                    ruby_handle.exception_type_error(),
                    "expected a string for body",
                ));
            }
        }

        // Get the read_body.
        let read_body_v = obj.get(Symbol::new("read_body"));
        if read_body_v.is_some() {
            // Get the read_body boolean.
            read_body = read_body_v.unwrap().to_bool();
        }
    }

    // Send the request.
    let res = builder.send();
    if res.is_err() {
        return Err(Error::new(
            ruby_handle.exception_runtime_error(),
            format!("failed to send request: {}", res.unwrap_err()),
        ));
    }

    // Get the status and headers.
    let res = res.unwrap();
    let status_code = res.status();
    let headers = RHash::new();
    for (key, value) in res.headers() {
        // Get the key and value.
        let key = RString::from_slice(key.as_str().as_bytes());
        let value = RString::from_slice(value.to_str().unwrap().as_bytes());

        // Set the header.
        _ = headers.aset(key, value);
    }

    // Try and get the body.
    let body: Option<RString>;
    if read_body {
        let res = res.bytes();
        if res.is_err() {
            return Err(Error::new(
                ruby_handle.exception_runtime_error(),
                format!("failed to get response body: {}", res.unwrap_err()),
            ));
        }

        // Get the body.
        let res = res.unwrap();
        body = Some(RString::from_slice(res.to_vec().as_slice()));
    } else {
        body = None;
    }

    // Return the response.
    let response = RHash::new();
    _ = response.aset(Symbol::new("status_code"), status_code.as_u16());
    _ = response.aset(Symbol::new("headers"), headers);
    _ = response.aset(Symbol::new("body"), body);
    Ok(response)
}

fn get(url: RString, obj: Option<RHash>) -> Result<RHash, Error> {
    do_request(url, obj, reqwest::Method::GET)
}

fn post(url: RString, obj: Option<RHash>) -> Result<RHash, Error> {
    do_request(url, obj, reqwest::Method::POST)
}

fn put(url: RString, obj: Option<RHash>) -> Result<RHash, Error> {
    do_request(url, obj, reqwest::Method::PUT)
}

fn delete(url: RString, obj: Option<RHash>) -> Result<RHash, Error> {
    do_request(url, obj, reqwest::Method::DELETE)
}

fn patch(url: RString, obj: Option<RHash>) -> Result<RHash, Error> {
    do_request(url, obj, reqwest::Method::PATCH)
}

#[magnus::init]
fn init() -> Result<(), Error> {
    // Setup the client.
    let client = reqwest::blocking::Client::new();
    let _ = CLIENT.set(client);

    // Setup the module.
    let module = define_module("Minigun")?;

    // Defines the HTTP methods.
    module.define_singleton_method("low_level_get", function!(get, 2))?;
    module.define_singleton_method("low_level_post", function!(post, 2))?;
    module.define_singleton_method("low_level_put", function!(put, 2))?;
    module.define_singleton_method("low_level_delete", function!(delete, 2))?;
    module.define_singleton_method("low_level_patch", function!(patch, 2))?;

    // Return a success.
    Ok(())
}
