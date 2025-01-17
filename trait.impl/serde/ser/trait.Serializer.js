(function() {
    var implementors = Object.fromEntries([["bincode",[["impl&lt;'a, W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>, O: <a class=\"trait\" href=\"bincode/config/trait.Options.html\" title=\"trait bincode::config::Options\">Options</a>&gt; <a class=\"trait\" href=\"serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a> for &amp;'a mut <a class=\"struct\" href=\"bincode/struct.Serializer.html\" title=\"struct bincode::Serializer\">Serializer</a>&lt;W, O&gt;"]]],["serde",[]],["serde_json",[["impl <a class=\"trait\" href=\"serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a> for <a class=\"struct\" href=\"serde_json/value/struct.Serializer.html\" title=\"struct serde_json::value::Serializer\">Serializer</a>"],["impl&lt;'a, W, F&gt; <a class=\"trait\" href=\"serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a> for &amp;'a mut <a class=\"struct\" href=\"serde_json/struct.Serializer.html\" title=\"struct serde_json::Serializer\">Serializer</a>&lt;W, F&gt;<div class=\"where\">where\n    W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>,\n    F: <a class=\"trait\" href=\"serde_json/ser/trait.Formatter.html\" title=\"trait serde_json::ser::Formatter\">Formatter</a>,</div>"]]],["serde_path_to_error",[["impl&lt;'a, 'b, S&gt; <a class=\"trait\" href=\"serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a> for <a class=\"struct\" href=\"serde_path_to_error/struct.Serializer.html\" title=\"struct serde_path_to_error::Serializer\">Serializer</a>&lt;'a, 'b, S&gt;<div class=\"where\">where\n    S: <a class=\"trait\" href=\"serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a>,</div>"]]],["serde_urlencoded",[["impl&lt;'input, 'output, Target&gt; <a class=\"trait\" href=\"serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a> for <a class=\"struct\" href=\"serde_urlencoded/struct.Serializer.html\" title=\"struct serde_urlencoded::Serializer\">Serializer</a>&lt;'input, 'output, Target&gt;<div class=\"where\">where\n    Target: 'output + <a class=\"trait\" href=\"form_urlencoded/trait.Target.html\" title=\"trait form_urlencoded::Target\">UrlEncodedTarget</a>,</div>"]]],["toml_edit",[["impl <a class=\"trait\" href=\"serde/ser/trait.Serializer.html\" title=\"trait serde::ser::Serializer\">Serializer</a> for <a class=\"struct\" href=\"toml_edit/ser/struct.ValueSerializer.html\" title=\"struct toml_edit::ser::ValueSerializer\">ValueSerializer</a>"]]]]);
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})()
//{"start":57,"fragment_lengths":[535,13,857,478,521,283]}