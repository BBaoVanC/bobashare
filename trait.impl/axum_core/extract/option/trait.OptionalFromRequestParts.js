(function() {
    var implementors = Object.fromEntries([["axum",[["impl&lt;S&gt; <a class=\"trait\" href=\"axum/extract/trait.OptionalFromRequestParts.html\" title=\"trait axum::extract::OptionalFromRequestParts\">OptionalFromRequestParts</a>&lt;S&gt; for <a class=\"struct\" href=\"axum/extract/struct.MatchedPath.html\" title=\"struct axum::extract::MatchedPath\">MatchedPath</a><div class=\"where\">where\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,</div>"],["impl&lt;T, S&gt; <a class=\"trait\" href=\"axum/extract/trait.OptionalFromRequestParts.html\" title=\"trait axum::extract::OptionalFromRequestParts\">OptionalFromRequestParts</a>&lt;S&gt; for <a class=\"struct\" href=\"axum/extract/struct.Path.html\" title=\"struct axum::extract::Path\">Path</a>&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"serde/de/trait.DeserializeOwned.html\" title=\"trait serde::de::DeserializeOwned\">DeserializeOwned</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,</div>"]]],["axum_extra",[["impl&lt;T, S&gt; <a class=\"trait\" href=\"axum_core/extract/option/trait.OptionalFromRequestParts.html\" title=\"trait axum_core::extract::option::OptionalFromRequestParts\">OptionalFromRequestParts</a>&lt;S&gt; for <a class=\"struct\" href=\"axum_extra/struct.TypedHeader.html\" title=\"struct axum_extra::TypedHeader\">TypedHeader</a>&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"headers_core/trait.Header.html\" title=\"trait headers_core::Header\">Header</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,</div>"]]]]);
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})()
//{"start":57,"fragment_lengths":[1541,793]}