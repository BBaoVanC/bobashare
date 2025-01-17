searchState.loadedDescShard("axum_core", 0, "Core types and traits for <code>axum</code>.\nAlias for a type-erased error type.\nErrors that can happen when using axum.\nExtension trait that adds additional methods to <code>Request</code>.\nExtension trait that adds additional methods to <code>Parts</code>.\nHTTP body utilities.\nTypes and traits for extracting data from requests.\nApply an extractor to this <code>Request</code>.\nApply an extractor to this <code>Parts</code>.\nApply a parts extractor to this <code>Request</code>.\nApply a parts extractor that requires some state to this …\nApply an extractor that requires some state to this <code>Request</code>…\nApply an extractor that requires some state to this <code>Parts</code>.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nConvert an <code>Error</code> back into the underlying boxed trait …\nConsumes the request, returning the body wrapped in …\nCreate a new <code>Error</code> from a boxable error.\nTypes and traits for generating responses.\nApply the default body limit.\nThe body type used in axum requests and responses.\nA stream of data frames.\nCreate an empty body.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreate a new <code>Body</code> from a <code>Stream</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConvert the body into a <code>Stream</code> of data frames.\nCreate a new <code>Body</code> that wraps another <code>http_body::Body</code>.\nLayer for configuring the default request body limit.\nUsed to do reference-to-value conversions thus not …\nTypes that can be created from requests.\nTypes that can be created from request parts.\nCustomize the behavior of <code>Option&lt;Self&gt;</code> as a <code>FromRequest</code> …\nCustomize the behavior of <code>Option&lt;Self&gt;</code> as a …\nIf the extractor fails, it will use this “rejection” …\nIf the extractor fails, it will use this “rejection” …\nIf the extractor fails it’ll use this “rejection” …\nIf the extractor fails it’ll use this “rejection” …\nType alias for <code>http::Request</code> whose body type defaults to …\nDisable the default request body limit.\nReturns the argument unchanged.\nConverts to this type from a reference to the input type.\nPerform the extraction.\nPerform the extraction.\nPerform the extraction.\nPerform the extraction.\nCalls <code>U::from(self)</code>.\nSet the default request body limit.\nRejection response types.\nRejection used for <code>Bytes</code>.\nRejection type for extractors that buffer the request …\nRejection type used when buffering the request into a …\nEncountered some other error when buffering the body.\nRejection used for <code>String</code>.\nEncountered an unknown error when buffering the body.\nGet the response body text used for this rejection.\nGet the response body text used for this rejection.\nGet the response body text used for this rejection.\nGet the response body text used for this rejection.\nGet the response body text used for this rejection.\nGet the response body text used for this rejection.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nGet the status code used for this rejection.\nGet the status code used for this rejection.\nGet the status code used for this rejection.\nGet the status code used for this rejection.\nGet the status code used for this rejection.\nGet the status code used for this rejection.\nAppend headers to a response.\nContains the error value\nThe type returned in the event of an error.\nAn <code>IntoResponse</code>-based error type\nTrait for generating responses.\nTrait for adding headers and extensions to a response.\nContains the success value\nType alias for <code>http::Response</code> whose body type defaults to …\nParts of a response.\nAn <code>IntoResponse</code>-based result type that uses <code>ErrorResponse</code> …\nError returned if converting a value to a header fails.\nGets a reference to the response extensions.\nGets a mutable reference to the response extensions.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGets a reference to the response headers.\nGets a mutable reference to the response headers.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreate a response.\nSet parts of the response")