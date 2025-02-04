(function() {
    var type_impls = Object.fromEntries([["chumsky",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3C%26%5BT%5D%3E-for-Stream%3C'a,+T,+Range%3Cusize%3E,+Box%3Cdyn+Iterator%3CItem+=+(T,+Range%3Cusize%3E)%3E%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/chumsky/stream.rs.html#286-296\">Source</a><a href=\"#impl-From%3C%26%5BT%5D%3E-for-Stream%3C'a,+T,+Range%3Cusize%3E,+Box%3Cdyn+Iterator%3CItem+=+(T,+Range%3Cusize%3E)%3E%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.slice.html\">[T]</a>&gt; for <a class=\"struct\" href=\"chumsky/stream/struct.Stream.html\" title=\"struct chumsky::stream::Stream\">Stream</a>&lt;'a, T, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/core/ops/range/struct.Range.html\" title=\"struct core::ops::range::Range\">Range</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.usize.html\">usize</a>&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = (T, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/core/ops/range/struct.Range.html\" title=\"struct core::ops::range::Range\">Range</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.usize.html\">usize</a>&gt;)&gt; + 'a&gt;&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/chumsky/stream.rs.html#289-295\">Source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(s: &amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.slice.html\">[T]</a>) -&gt; Self</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<&'a [T]>","chumsky::stream::BoxStream"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3C%26%5BT;+N%5D%3E-for-Stream%3C'a,+T,+Range%3Cusize%3E,+Box%3Cdyn+Iterator%3CItem+=+(T,+Range%3Cusize%3E)%3E%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/chumsky/stream.rs.html#325-334\">Source</a><a href=\"#impl-From%3C%26%5BT;+N%5D%3E-for-Stream%3C'a,+T,+Range%3Cusize%3E,+Box%3Cdyn+Iterator%3CItem+=+(T,+Range%3Cusize%3E)%3E%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.array.html\">[T; N]</a>&gt; for <a class=\"struct\" href=\"chumsky/stream/struct.Stream.html\" title=\"struct chumsky::stream::Stream\">Stream</a>&lt;'a, T, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/core/ops/range/struct.Range.html\" title=\"struct core::ops::range::Range\">Range</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.usize.html\">usize</a>&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = (T, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/core/ops/range/struct.Range.html\" title=\"struct core::ops::range::Range\">Range</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.usize.html\">usize</a>&gt;)&gt; + 'a&gt;&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/chumsky/stream.rs.html#328-333\">Source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(s: &amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.array.html\">[T; N]</a>) -&gt; Self</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<&'a [T; N]>","chumsky::stream::BoxStream"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3C%26str%3E-for-Stream%3C'a,+char,+Range%3Cusize%3E,+Box%3Cdyn+Iterator%3CItem+=+(char,+Range%3Cusize%3E)%3E%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/chumsky/stream.rs.html#258-270\">Source</a><a href=\"#impl-From%3C%26str%3E-for-Stream%3C'a,+char,+Range%3Cusize%3E,+Box%3Cdyn+Iterator%3CItem+=+(char,+Range%3Cusize%3E)%3E%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"chumsky/stream/struct.Stream.html\" title=\"struct chumsky::stream::Stream\">Stream</a>&lt;'a, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.char.html\">char</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/core/ops/range/struct.Range.html\" title=\"struct core::ops::range::Range\">Range</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.usize.html\">usize</a>&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = (<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.char.html\">char</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/core/ops/range/struct.Range.html\" title=\"struct core::ops::range::Range\">Range</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.usize.html\">usize</a>&gt;)&gt; + 'a&gt;&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/chumsky/stream.rs.html#263-269\">Source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(s: &amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.str.html\">str</a>) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Please note that Chumsky currently uses character indices and not byte offsets in this impl. This is likely to\nchange in the future. If you wish to use byte offsets, you can do so with <a href=\"chumsky/stream/struct.Stream.html#method.from_iter\" title=\"associated function chumsky::stream::Stream::from_iter\"><code>Stream::from_iter</code></a>.</p>\n</div></details></div></details>","From<&'a str>","chumsky::stream::BoxStream"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3C%5BT;+N%5D%3E-for-Stream%3C'a,+T,+Range%3Cusize%3E,+Box%3Cdyn+Iterator%3CItem+=+(T,+Range%3Cusize%3E)%3E%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/chumsky/stream.rs.html#310-323\">Source</a><a href=\"#impl-From%3C%5BT;+N%5D%3E-for-Stream%3C'a,+T,+Range%3Cusize%3E,+Box%3Cdyn+Iterator%3CItem+=+(T,+Range%3Cusize%3E)%3E%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + 'a, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.array.html\">[T; N]</a>&gt; for <a class=\"struct\" href=\"chumsky/stream/struct.Stream.html\" title=\"struct chumsky::stream::Stream\">Stream</a>&lt;'a, T, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/core/ops/range/struct.Range.html\" title=\"struct core::ops::range::Range\">Range</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.usize.html\">usize</a>&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = (T, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/core/ops/range/struct.Range.html\" title=\"struct core::ops::range::Range\">Range</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.usize.html\">usize</a>&gt;)&gt; + 'a&gt;&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/chumsky/stream.rs.html#313-322\">Source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(s: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.array.html\">[T; N]</a>) -&gt; Self</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<[T; N]>","chumsky::stream::BoxStream"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CString%3E-for-Stream%3C'a,+char,+Range%3Cusize%3E,+Box%3Cdyn+Iterator%3CItem+=+(char,+Range%3Cusize%3E)%3E%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/chumsky/stream.rs.html#272-284\">Source</a><a href=\"#impl-From%3CString%3E-for-Stream%3C'a,+char,+Range%3Cusize%3E,+Box%3Cdyn+Iterator%3CItem+=+(char,+Range%3Cusize%3E)%3E%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"struct\" href=\"chumsky/stream/struct.Stream.html\" title=\"struct chumsky::stream::Stream\">Stream</a>&lt;'a, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.char.html\">char</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/core/ops/range/struct.Range.html\" title=\"struct core::ops::range::Range\">Range</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.usize.html\">usize</a>&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = (<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.char.html\">char</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/core/ops/range/struct.Range.html\" title=\"struct core::ops::range::Range\">Range</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.usize.html\">usize</a>&gt;)&gt;&gt;&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/chumsky/stream.rs.html#277-283\">Source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(s: <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Please note that Chumsky currently uses character indices and not byte offsets in this impl. This is likely to\nchange in the future. If you wish to use byte offsets, you can do so with <a href=\"chumsky/stream/struct.Stream.html#method.from_iter\" title=\"associated function chumsky::stream::Stream::from_iter\"><code>Stream::from_iter</code></a>.</p>\n</div></details></div></details>","From<String>","chumsky::stream::BoxStream"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CVec%3CT%3E%3E-for-Stream%3C'a,+T,+Range%3Cusize%3E,+Box%3Cdyn+Iterator%3CItem+=+(T,+Range%3Cusize%3E)%3E%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/chumsky/stream.rs.html#298-308\">Source</a><a href=\"#impl-From%3CVec%3CT%3E%3E-for-Stream%3C'a,+T,+Range%3Cusize%3E,+Box%3Cdyn+Iterator%3CItem+=+(T,+Range%3Cusize%3E)%3E%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + 'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"chumsky/stream/struct.Stream.html\" title=\"struct chumsky::stream::Stream\">Stream</a>&lt;'a, T, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/core/ops/range/struct.Range.html\" title=\"struct core::ops::range::Range\">Range</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.usize.html\">usize</a>&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = (T, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/core/ops/range/struct.Range.html\" title=\"struct core::ops::range::Range\">Range</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.usize.html\">usize</a>&gt;)&gt; + 'a&gt;&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/chumsky/stream.rs.html#301-307\">Source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(s: <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;T&gt;) -&gt; Self</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<Vec<T>>","chumsky::stream::BoxStream"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Stream%3C'a,+I,+S,+Iter%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/chumsky/stream.rs.html#52-79\">Source</a><a href=\"#impl-Stream%3C'a,+I,+S,+Iter%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, I, S: <a class=\"trait\" href=\"chumsky/span/trait.Span.html\" title=\"trait chumsky::span::Span\">Span</a>, Iter: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.tuple.html\">(I, S)</a>&gt;&gt; <a class=\"struct\" href=\"chumsky/stream/struct.Stream.html\" title=\"struct chumsky::stream::Stream\">Stream</a>&lt;'a, I, S, Iter&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_iter\" class=\"method\"><a class=\"src rightside\" href=\"src/chumsky/stream.rs.html#58-66\">Source</a><h4 class=\"code-header\">pub fn <a href=\"chumsky/stream/struct.Stream.html#tymethod.from_iter\" class=\"fn\">from_iter</a>(eoi: S, iter: Iter) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Create a new stream from an iterator of <code>(Token, Span)</code> pairs. A span representing the end of input must also\nbe provided.</p>\n<p>There is no requirement that spans must map exactly to the position of inputs in the stream, but they should\nbe non-overlapping and should appear in a monotonically-increasing order.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.fetch_tokens\" class=\"method\"><a class=\"src rightside\" href=\"src/chumsky/stream.rs.html#72-78\">Source</a><h4 class=\"code-header\">pub fn <a href=\"chumsky/stream/struct.Stream.html#tymethod.fetch_tokens\" class=\"fn\">fetch_tokens</a>(&amp;mut self) -&gt; impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/iter/traits/iterator/trait.Iterator.html\" title=\"trait core::iter::traits::iterator::Iterator\">Iterator</a>&lt;Item = <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.tuple.html\">(I, S)</a>&gt; + '_<div class=\"where\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.tuple.html\">(I, S)</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div></h4></section></summary><div class=\"docblock\"><p>Eagerly evaluate the token stream, returning an iterator over the tokens in it (but without modifying the\nstream’s state so that it can still be used for parsing).</p>\n<p>This is most useful when you wish to check the input of a parser during debugging.</p>\n</div></details></div></details>",0,"chumsky::stream::BoxStream"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[19562]}