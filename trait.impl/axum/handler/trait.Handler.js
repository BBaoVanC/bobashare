(function() {
    var implementors = Object.fromEntries([["axum",[]],["axum_extra",[["impl&lt;H, T, S&gt; <a class=\"trait\" href=\"axum/handler/trait.Handler.html\" title=\"trait axum::handler::Handler\">Handler</a>&lt;T, S&gt; for <a class=\"struct\" href=\"axum_extra/handler/struct.IntoHandler.html\" title=\"struct axum_extra::handler::IntoHandler\">IntoHandler</a>&lt;H, T, S&gt;<div class=\"where\">where\n    H: <a class=\"trait\" href=\"axum_extra/handler/trait.HandlerCallWithExtractors.html\" title=\"trait axum_extra::handler::HandlerCallWithExtractors\">HandlerCallWithExtractors</a>&lt;T, S&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,\n    T: <a class=\"trait\" href=\"axum_core/extract/trait.FromRequest.html\" title=\"trait axum_core::extract::FromRequest\">FromRequest</a>&lt;S&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,\n    T::<a class=\"associatedtype\" href=\"axum_core/extract/trait.FromRequest.html#associatedtype.Rejection\" title=\"type axum_core::extract::FromRequest::Rejection\">Rejection</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</div>"],["impl&lt;S, L, R, Lt, Rt, M&gt; <a class=\"trait\" href=\"axum/handler/trait.Handler.html\" title=\"trait axum::handler::Handler\">Handler</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.0/std/primitive.tuple.html\">(M, Lt, Rt)</a>, S&gt; for <a class=\"struct\" href=\"axum_extra/handler/struct.Or.html\" title=\"struct axum_extra::handler::Or\">Or</a>&lt;L, R, Lt, Rt, S&gt;<div class=\"where\">where\n    L: <a class=\"trait\" href=\"axum_extra/handler/trait.HandlerCallWithExtractors.html\" title=\"trait axum_extra::handler::HandlerCallWithExtractors\">HandlerCallWithExtractors</a>&lt;Lt, S&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,\n    R: <a class=\"trait\" href=\"axum_extra/handler/trait.HandlerCallWithExtractors.html\" title=\"trait axum_extra::handler::HandlerCallWithExtractors\">HandlerCallWithExtractors</a>&lt;Rt, S&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,\n    Lt: <a class=\"trait\" href=\"axum_core/extract/trait.FromRequestParts.html\" title=\"trait axum_core::extract::FromRequestParts\">FromRequestParts</a>&lt;S&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,\n    Rt: <a class=\"trait\" href=\"axum_core/extract/trait.FromRequest.html\" title=\"trait axum_core::extract::FromRequest\">FromRequest</a>&lt;S, M&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,\n    Lt::<a class=\"associatedtype\" href=\"axum_core/extract/trait.FromRequestParts.html#associatedtype.Rejection\" title=\"type axum_core::extract::FromRequestParts::Rejection\">Rejection</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,\n    Rt::<a class=\"associatedtype\" href=\"axum_core/extract/trait.FromRequest.html#associatedtype.Rejection\" title=\"type axum_core::extract::FromRequest::Rejection\">Rejection</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</div>"]]]]);
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})()
//{"start":57,"fragment_lengths":[11,5077]}