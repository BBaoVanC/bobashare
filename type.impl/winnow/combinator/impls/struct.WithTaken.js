(function() {
    var type_impls = Object.fromEntries([["winnow",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Parser%3CI,+(O,+%3CI+as+Stream%3E::Slice),+E%3E-for-WithTaken%3CF,+I,+O,+E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/winnow/combinator/impls.rs.html#421-439\">Source</a><a href=\"#impl-Parser%3CI,+(O,+%3CI+as+Stream%3E::Slice),+E%3E-for-WithTaken%3CF,+I,+O,+E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;F, I, O, E&gt; <a class=\"trait\" href=\"winnow/trait.Parser.html\" title=\"trait winnow::Parser\">Parser</a>&lt;I, (O, &lt;I as <a class=\"trait\" href=\"winnow/stream/trait.Stream.html\" title=\"trait winnow::stream::Stream\">Stream</a>&gt;::<a class=\"associatedtype\" href=\"winnow/stream/trait.Stream.html#associatedtype.Slice\" title=\"type winnow::stream::Stream::Slice\">Slice</a>), E&gt; for <a class=\"struct\" href=\"winnow/combinator/impls/struct.WithTaken.html\" title=\"struct winnow::combinator::impls::WithTaken\">WithTaken</a>&lt;F, I, O, E&gt;<div class=\"where\">where\n    F: <a class=\"trait\" href=\"winnow/trait.Parser.html\" title=\"trait winnow::Parser\">Parser</a>&lt;I, O, E&gt;,\n    I: <a class=\"trait\" href=\"winnow/stream/trait.Stream.html\" title=\"trait winnow::stream::Stream\">Stream</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.parse_next\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winnow/combinator/impls.rs.html#427-438\">Source</a><a href=\"#method.parse_next\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"winnow/trait.Parser.html#tymethod.parse_next\" class=\"fn\">parse_next</a>(&amp;mut self, input: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.0/std/primitive.reference.html\">&amp;mut I</a>) -&gt; <a class=\"type\" href=\"winnow/error/type.PResult.html\" title=\"type winnow::error::PResult\">PResult</a>&lt;(O, &lt;I as <a class=\"trait\" href=\"winnow/stream/trait.Stream.html\" title=\"trait winnow::stream::Stream\">Stream</a>&gt;::<a class=\"associatedtype\" href=\"winnow/stream/trait.Stream.html#associatedtype.Slice\" title=\"type winnow::stream::Stream::Slice\">Slice</a>), E&gt;</h4></section></summary><div class='docblock'>Take tokens from the <a href=\"winnow/stream/trait.Stream.html\" title=\"trait winnow::stream::Stream\"><code>Stream</code></a>, turning it into the output <a href=\"winnow/trait.Parser.html#tymethod.parse_next\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.parse\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winnow/parser.rs.html#51-74\">Source</a><a href=\"#method.parse\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"winnow/trait.Parser.html#method.parse\" class=\"fn\">parse</a>(&amp;mut self, input: I) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;O, <a class=\"struct\" href=\"winnow/error/struct.ParseError.html\" title=\"struct winnow::error::ParseError\">ParseError</a>&lt;I, E&gt;&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    I: <a class=\"trait\" href=\"winnow/stream/trait.Stream.html\" title=\"trait winnow::stream::Stream\">Stream</a> + <a class=\"trait\" href=\"winnow/stream/trait.StreamIsPartial.html\" title=\"trait winnow::stream::StreamIsPartial\">StreamIsPartial</a>,\n    E: <a class=\"trait\" href=\"winnow/error/trait.ParserError.html\" title=\"trait winnow::error::ParserError\">ParserError</a>&lt;I&gt;,</div></h4></section></summary><div class='docblock'>Parse all of <code>input</code>, generating <code>O</code> from it</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.parse_peek\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winnow/parser.rs.html#98-103\">Source</a><a href=\"#method.parse_peek\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"winnow/trait.Parser.html#method.parse_peek\" class=\"fn\">parse_peek</a>(&amp;mut self, input: I) -&gt; <a class=\"type\" href=\"winnow/error/type.IResult.html\" title=\"type winnow::error::IResult\">IResult</a>&lt;I, O, E&gt;</h4></section></summary><div class='docblock'>Take tokens from the <a href=\"winnow/stream/trait.Stream.html\" title=\"trait winnow::stream::Stream\"><code>Stream</code></a>, turning it into the output <a href=\"winnow/trait.Parser.html#method.parse_peek\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.by_ref\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winnow/parser.rs.html#149-154\">Source</a><a href=\"#method.by_ref\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"winnow/trait.Parser.html#method.by_ref\" class=\"fn\">by_ref</a>(&amp;mut self) -&gt; <a class=\"struct\" href=\"winnow/combinator/impls/struct.ByRef.html\" title=\"struct winnow::combinator::impls::ByRef\">ByRef</a>&lt;'_, Self&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Treat <code>&amp;mut Self</code> as a parser <a href=\"winnow/trait.Parser.html#method.by_ref\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.value\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winnow/parser.rs.html#173-185\">Source</a><a href=\"#method.value\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"winnow/trait.Parser.html#method.value\" class=\"fn\">value</a>&lt;O2&gt;(self, val: O2) -&gt; <a class=\"struct\" href=\"winnow/combinator/impls/struct.Value.html\" title=\"struct winnow::combinator::impls::Value\">Value</a>&lt;Self, I, O, O2, E&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    O2: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div></h4></section></summary><div class='docblock'>Produce the provided value <a href=\"winnow/trait.Parser.html#method.value\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.default_value\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winnow/parser.rs.html#203-215\">Source</a><a href=\"#method.default_value\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"winnow/trait.Parser.html#method.default_value\" class=\"fn\">default_value</a>&lt;O2&gt;(self) -&gt; <a class=\"struct\" href=\"winnow/combinator/impls/struct.DefaultValue.html\" title=\"struct winnow::combinator::impls::DefaultValue\">DefaultValue</a>&lt;Self, I, O, O2, E&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    O2: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,</div></h4></section></summary><div class='docblock'>Produce a type’s default value <a href=\"winnow/trait.Parser.html#method.default_value\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.void\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winnow/parser.rs.html#233-243\">Source</a><a href=\"#method.void\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"winnow/trait.Parser.html#method.void\" class=\"fn\">void</a>(self) -&gt; <a class=\"struct\" href=\"winnow/combinator/impls/struct.Void.html\" title=\"struct winnow::combinator::impls::Void\">Void</a>&lt;Self, I, O, E&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Discards the output of the <code>Parser</code> <a href=\"winnow/trait.Parser.html#method.void\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.output_into\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winnow/parser.rs.html#267-279\">Source</a><a href=\"#method.output_into\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"winnow/trait.Parser.html#method.output_into\" class=\"fn\">output_into</a>&lt;O2&gt;(self) -&gt; <a class=\"struct\" href=\"winnow/combinator/impls/struct.OutputInto.html\" title=\"struct winnow::combinator::impls::OutputInto\">OutputInto</a>&lt;Self, I, O, O2, E&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    O: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;O2&gt;,</div></h4></section></summary><div class='docblock'>Convert the parser’s output to another type using <a href=\"https://doc.rust-lang.org/1.84.0/core/convert/trait.From.html\" title=\"trait core::convert::From\"><code>std::convert::From</code></a> <a href=\"winnow/trait.Parser.html#method.output_into\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.take\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winnow/parser.rs.html#300-311\">Source</a><a href=\"#method.take\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"winnow/trait.Parser.html#method.take\" class=\"fn\">take</a>(self) -&gt; <a class=\"struct\" href=\"winnow/combinator/impls/struct.Take.html\" title=\"struct winnow::combinator::impls::Take\">Take</a>&lt;Self, I, O, E&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    I: <a class=\"trait\" href=\"winnow/stream/trait.Stream.html\" title=\"trait winnow::stream::Stream\">Stream</a>,</div></h4></section></summary><div class='docblock'>Produce the consumed input as produced value. <a href=\"winnow/trait.Parser.html#method.take\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.recognize\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winnow/parser.rs.html#316-327\">Source</a><a href=\"#method.recognize\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"winnow/trait.Parser.html#method.recognize\" class=\"fn\">recognize</a>(self) -&gt; <a class=\"struct\" href=\"winnow/combinator/impls/struct.Take.html\" title=\"struct winnow::combinator::impls::Take\">Take</a>&lt;Self, I, O, E&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    I: <a class=\"trait\" href=\"winnow/stream/trait.Stream.html\" title=\"trait winnow::stream::Stream\">Stream</a>,</div></h4></section></summary><span class=\"item-info\"><div class=\"stab deprecated\"><span class=\"emoji\">👎</span><span>Deprecated since 0.6.14: Replaced with <code>Parser::take</code></span></div></span><div class='docblock'>Replaced with <a href=\"winnow/trait.Parser.html#method.take\" title=\"method winnow::Parser::take\"><code>Parser::take</code></a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.with_taken\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winnow/parser.rs.html#368-379\">Source</a><a href=\"#method.with_taken\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"winnow/trait.Parser.html#method.with_taken\" class=\"fn\">with_taken</a>(self) -&gt; <a class=\"struct\" href=\"winnow/combinator/impls/struct.WithTaken.html\" title=\"struct winnow::combinator::impls::WithTaken\">WithTaken</a>&lt;Self, I, O, E&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    I: <a class=\"trait\" href=\"winnow/stream/trait.Stream.html\" title=\"trait winnow::stream::Stream\">Stream</a>,</div></h4></section></summary><div class='docblock'>Produce the consumed input with the output <a href=\"winnow/trait.Parser.html#method.with_taken\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.with_recognized\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winnow/parser.rs.html#384-395\">Source</a><a href=\"#method.with_recognized\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"winnow/trait.Parser.html#method.with_recognized\" class=\"fn\">with_recognized</a>(self) -&gt; <a class=\"struct\" href=\"winnow/combinator/impls/struct.WithTaken.html\" title=\"struct winnow::combinator::impls::WithTaken\">WithTaken</a>&lt;Self, I, O, E&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    I: <a class=\"trait\" href=\"winnow/stream/trait.Stream.html\" title=\"trait winnow::stream::Stream\">Stream</a>,</div></h4></section></summary><span class=\"item-info\"><div class=\"stab deprecated\"><span class=\"emoji\">👎</span><span>Deprecated since 0.6.14: Replaced with <code>Parser::with_taken</code></span></div></span><div class='docblock'>Replaced with <a href=\"winnow/trait.Parser.html#method.with_taken\" title=\"method winnow::Parser::with_taken\"><code>Parser::with_taken</code></a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.span\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winnow/parser.rs.html#414-425\">Source</a><a href=\"#method.span\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"winnow/trait.Parser.html#method.span\" class=\"fn\">span</a>(self) -&gt; <a class=\"struct\" href=\"winnow/combinator/impls/struct.Span.html\" title=\"struct winnow::combinator::impls::Span\">Span</a>&lt;Self, I, O, E&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    I: <a class=\"trait\" href=\"winnow/stream/trait.Stream.html\" title=\"trait winnow::stream::Stream\">Stream</a> + <a class=\"trait\" href=\"winnow/stream/trait.Location.html\" title=\"trait winnow::stream::Location\">Location</a>,</div></h4></section></summary><div class='docblock'>Produce the location of the consumed input as produced value. <a href=\"winnow/trait.Parser.html#method.span\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.with_span\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winnow/parser.rs.html#468-479\">Source</a><a href=\"#method.with_span\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"winnow/trait.Parser.html#method.with_span\" class=\"fn\">with_span</a>(self) -&gt; <a class=\"struct\" href=\"winnow/combinator/impls/struct.WithSpan.html\" title=\"struct winnow::combinator::impls::WithSpan\">WithSpan</a>&lt;Self, I, O, E&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    I: <a class=\"trait\" href=\"winnow/stream/trait.Stream.html\" title=\"trait winnow::stream::Stream\">Stream</a> + <a class=\"trait\" href=\"winnow/stream/trait.Location.html\" title=\"trait winnow::stream::Location\">Location</a>,</div></h4></section></summary><div class='docblock'>Produce the location of consumed input with the output <a href=\"winnow/trait.Parser.html#method.with_span\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.map\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winnow/parser.rs.html#500-513\">Source</a><a href=\"#method.map\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"winnow/trait.Parser.html#method.map\" class=\"fn\">map</a>&lt;G, O2&gt;(self, map: G) -&gt; <a class=\"struct\" href=\"winnow/combinator/impls/struct.Map.html\" title=\"struct winnow::combinator::impls::Map\">Map</a>&lt;Self, G, I, O, O2, E&gt;<div class=\"where\">where\n    G: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(O) -&gt; O2,\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Maps a function over the output of a parser <a href=\"winnow/trait.Parser.html#method.map\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_map\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winnow/parser.rs.html#537-553\">Source</a><a href=\"#method.try_map\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"winnow/trait.Parser.html#method.try_map\" class=\"fn\">try_map</a>&lt;G, O2, E2&gt;(self, map: G) -&gt; <a class=\"struct\" href=\"winnow/combinator/impls/struct.TryMap.html\" title=\"struct winnow::combinator::impls::TryMap\">TryMap</a>&lt;Self, G, I, O, O2, E, E2&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    G: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(O) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;O2, E2&gt;,\n    I: <a class=\"trait\" href=\"winnow/stream/trait.Stream.html\" title=\"trait winnow::stream::Stream\">Stream</a>,\n    E: <a class=\"trait\" href=\"winnow/error/trait.FromExternalError.html\" title=\"trait winnow::error::FromExternalError\">FromExternalError</a>&lt;I, E2&gt;,</div></h4></section></summary><div class='docblock'>Applies a function returning a <code>Result</code> over the output of a parser. <a href=\"winnow/trait.Parser.html#method.try_map\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.verify_map\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winnow/parser.rs.html#580-595\">Source</a><a href=\"#method.verify_map\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"winnow/trait.Parser.html#method.verify_map\" class=\"fn\">verify_map</a>&lt;G, O2&gt;(self, map: G) -&gt; <a class=\"struct\" href=\"winnow/combinator/impls/struct.VerifyMap.html\" title=\"struct winnow::combinator::impls::VerifyMap\">VerifyMap</a>&lt;Self, G, I, O, O2, E&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    G: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(O) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;O2&gt;,\n    I: <a class=\"trait\" href=\"winnow/stream/trait.Stream.html\" title=\"trait winnow::stream::Stream\">Stream</a>,\n    E: <a class=\"trait\" href=\"winnow/error/trait.ParserError.html\" title=\"trait winnow::error::ParserError\">ParserError</a>&lt;I&gt;,</div></h4></section></summary><div class='docblock'>Apply both <a href=\"winnow/trait.Parser.html#method.verify\" title=\"method winnow::Parser::verify\"><code>Parser::verify</code></a> and <a href=\"winnow/trait.Parser.html#method.map\" title=\"method winnow::Parser::map\"><code>Parser::map</code></a>. <a href=\"winnow/trait.Parser.html#method.verify_map\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.flat_map\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winnow/parser.rs.html#630-645\">Source</a><a href=\"#method.flat_map\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"winnow/trait.Parser.html#method.flat_map\" class=\"fn\">flat_map</a>&lt;G, H, O2&gt;(self, map: G) -&gt; <a class=\"struct\" href=\"winnow/combinator/impls/struct.FlatMap.html\" title=\"struct winnow::combinator::impls::FlatMap\">FlatMap</a>&lt;Self, G, H, I, O, O2, E&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    G: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(O) -&gt; H,\n    H: <a class=\"trait\" href=\"winnow/trait.Parser.html\" title=\"trait winnow::Parser\">Parser</a>&lt;I, O2, E&gt;,</div></h4></section></summary><div class='docblock'>Creates a parser from the output of this one <a href=\"winnow/trait.Parser.html#method.flat_map\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.and_then\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winnow/parser.rs.html#665-680\">Source</a><a href=\"#method.and_then\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"winnow/trait.Parser.html#method.and_then\" class=\"fn\">and_then</a>&lt;G, O2&gt;(self, inner: G) -&gt; <a class=\"struct\" href=\"winnow/combinator/impls/struct.AndThen.html\" title=\"struct winnow::combinator::impls::AndThen\">AndThen</a>&lt;Self, G, I, O, O2, E&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    G: <a class=\"trait\" href=\"winnow/trait.Parser.html\" title=\"trait winnow::Parser\">Parser</a>&lt;O, O2, E&gt;,\n    O: <a class=\"trait\" href=\"winnow/stream/trait.StreamIsPartial.html\" title=\"trait winnow::stream::StreamIsPartial\">StreamIsPartial</a>,\n    I: <a class=\"trait\" href=\"winnow/stream/trait.Stream.html\" title=\"trait winnow::stream::Stream\">Stream</a>,</div></h4></section></summary><div class='docblock'>Applies a second parser over the output of the first one <a href=\"winnow/trait.Parser.html#method.and_then\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.parse_to\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winnow/parser.rs.html#703-717\">Source</a><a href=\"#method.parse_to\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"winnow/trait.Parser.html#method.parse_to\" class=\"fn\">parse_to</a>&lt;O2&gt;(self) -&gt; <a class=\"struct\" href=\"winnow/combinator/impls/struct.ParseTo.html\" title=\"struct winnow::combinator::impls::ParseTo\">ParseTo</a>&lt;Self, I, O, O2, E&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    I: <a class=\"trait\" href=\"winnow/stream/trait.Stream.html\" title=\"trait winnow::stream::Stream\">Stream</a>,\n    O: <a class=\"trait\" href=\"winnow/stream/trait.ParseSlice.html\" title=\"trait winnow::stream::ParseSlice\">ParseSlice</a>&lt;O2&gt;,\n    E: <a class=\"trait\" href=\"winnow/error/trait.ParserError.html\" title=\"trait winnow::error::ParserError\">ParserError</a>&lt;I&gt;,</div></h4></section></summary><div class='docblock'>Apply <a href=\"https://doc.rust-lang.org/1.84.0/core/str/traits/trait.FromStr.html\" title=\"trait core::str::traits::FromStr\"><code>std::str::FromStr</code></a> to the output of the parser <a href=\"winnow/trait.Parser.html#method.parse_to\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.verify\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winnow/parser.rs.html#741-758\">Source</a><a href=\"#method.verify\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"winnow/trait.Parser.html#method.verify\" class=\"fn\">verify</a>&lt;G, O2&gt;(self, filter: G) -&gt; <a class=\"struct\" href=\"winnow/combinator/impls/struct.Verify.html\" title=\"struct winnow::combinator::impls::Verify\">Verify</a>&lt;Self, G, I, O, O2, E&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    G: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.0/std/primitive.reference.html\">&amp;O2</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.0/std/primitive.bool.html\">bool</a>,\n    I: <a class=\"trait\" href=\"winnow/stream/trait.Stream.html\" title=\"trait winnow::stream::Stream\">Stream</a>,\n    O: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;O2&gt;,\n    O2: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    E: <a class=\"trait\" href=\"winnow/error/trait.ParserError.html\" title=\"trait winnow::error::ParserError\">ParserError</a>&lt;I&gt;,</div></h4></section></summary><div class='docblock'>Returns the output of the child parser if it satisfies a verification function. <a href=\"winnow/trait.Parser.html#method.verify\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.context\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winnow/parser.rs.html#766-780\">Source</a><a href=\"#method.context\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"winnow/trait.Parser.html#method.context\" class=\"fn\">context</a>&lt;C&gt;(self, context: C) -&gt; <a class=\"struct\" href=\"winnow/combinator/impls/struct.Context.html\" title=\"struct winnow::combinator::impls::Context\">Context</a>&lt;Self, I, O, E, C&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    I: <a class=\"trait\" href=\"winnow/stream/trait.Stream.html\" title=\"trait winnow::stream::Stream\">Stream</a>,\n    E: <a class=\"trait\" href=\"winnow/error/trait.AddContext.html\" title=\"trait winnow::error::AddContext\">AddContext</a>&lt;I, C&gt;,\n    C: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,</div></h4></section></summary><div class='docblock'>If parsing fails, add context to the error <a href=\"winnow/trait.Parser.html#method.context\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.complete_err\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winnow/parser.rs.html#798-803\">Source</a><a href=\"#method.complete_err\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"winnow/trait.Parser.html#method.complete_err\" class=\"fn\">complete_err</a>(self) -&gt; <a class=\"struct\" href=\"winnow/combinator/impls/struct.CompleteErr.html\" title=\"struct winnow::combinator::impls::CompleteErr\">CompleteErr</a>&lt;Self&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h4></section></summary><div class='docblock'>Transforms <a href=\"winnow/error/enum.ErrMode.html#variant.Incomplete\" title=\"variant winnow::error::ErrMode::Incomplete\"><code>Incomplete</code></a> into <a href=\"winnow/error/enum.ErrMode.html#variant.Backtrack\" title=\"variant winnow::error::ErrMode::Backtrack\"><code>Backtrack</code></a> <a href=\"winnow/trait.Parser.html#method.complete_err\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.err_into\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/winnow/parser.rs.html#807-819\">Source</a><a href=\"#method.err_into\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"winnow/trait.Parser.html#method.err_into\" class=\"fn\">err_into</a>&lt;E2&gt;(self) -&gt; <a class=\"struct\" href=\"winnow/combinator/impls/struct.ErrInto.html\" title=\"struct winnow::combinator::impls::ErrInto\">ErrInto</a>&lt;Self, I, O, E, E2&gt;<div class=\"where\">where\n    Self: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    E: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;E2&gt;,</div></h4></section></summary><div class='docblock'>Convert the parser’s error to another type using <a href=\"https://doc.rust-lang.org/1.84.0/core/convert/trait.From.html\" title=\"trait core::convert::From\"><code>std::convert::From</code></a></div></details></div></details>","Parser<I, (O, <I as Stream>::Slice), E>","winnow::combinator::impls::WithRecognized"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[30238]}