(function() {var type_impls = {
"xcb":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ValueError\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-de3bc13b25639774/out/xproto.rs.html#141-197\">source</a><a href=\"#impl-ValueError\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"struct\" href=\"xcb/x/struct.ValueError.html\" title=\"struct xcb::x::ValueError\">ValueError</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.response_type\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-de3bc13b25639774/out/xproto.rs.html#150-156\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.ValueError.html#tymethod.response_type\" class=\"fn\">response_type</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.u8.html\">u8</a></h4></section><section id=\"method.error_code\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-de3bc13b25639774/out/xproto.rs.html#158-164\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.ValueError.html#tymethod.error_code\" class=\"fn\">error_code</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.u8.html\">u8</a></h4></section><section id=\"method.sequence\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-de3bc13b25639774/out/xproto.rs.html#166-172\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.ValueError.html#tymethod.sequence\" class=\"fn\">sequence</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.u16.html\">u16</a></h4></section><section id=\"method.bad_value\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-de3bc13b25639774/out/xproto.rs.html#174-180\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.ValueError.html#tymethod.bad_value\" class=\"fn\">bad_value</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.u32.html\">u32</a></h4></section><section id=\"method.minor_opcode\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-de3bc13b25639774/out/xproto.rs.html#182-188\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.ValueError.html#tymethod.minor_opcode\" class=\"fn\">minor_opcode</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.u16.html\">u16</a></h4></section><section id=\"method.major_opcode\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-de3bc13b25639774/out/xproto.rs.html#190-196\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.ValueError.html#tymethod.major_opcode\" class=\"fn\">major_opcode</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.u8.html\">u8</a></h4></section></div></details>",0,"xcb::xproto::WindowError","xcb::xproto::PixmapError","xcb::xproto::AtomError","xcb::xproto::CursorError","xcb::xproto::FontError","xcb::xproto::DrawableError","xcb::xproto::ColormapError","xcb::xproto::GContextError","xcb::xproto::IdChoiceError","xcb::shm::BadSegError"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Raw%3Cxcb_generic_error_t%3E-for-ValueError\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-de3bc13b25639774/out/xproto.rs.html#125-133\">source</a><a href=\"#impl-Raw%3Cxcb_generic_error_t%3E-for-ValueError\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"xcb/trait.Raw.html\" title=\"trait xcb::Raw\">Raw</a>&lt;<a class=\"struct\" href=\"xcb/ffi/struct.xcb_generic_error_t.html\" title=\"struct xcb::ffi::xcb_generic_error_t\">xcb_generic_error_t</a>&gt; for <a class=\"struct\" href=\"xcb/x/struct.ValueError.html\" title=\"struct xcb::x::ValueError\">ValueError</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_raw\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-de3bc13b25639774/out/xproto.rs.html#126-128\">source</a><a href=\"#method.from_raw\" class=\"anchor\">§</a><h4 class=\"code-header\">unsafe fn <a href=\"xcb/trait.Raw.html#tymethod.from_raw\" class=\"fn\">from_raw</a>(raw: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.pointer.html\">*mut </a><a class=\"struct\" href=\"xcb/ffi/struct.xcb_generic_error_t.html\" title=\"struct xcb::ffi::xcb_generic_error_t\">xcb_generic_error_t</a>) -&gt; Self</h4></section></summary><div class='docblock'>Build <code>Self</code> from a raw pointer <a href=\"xcb/trait.Raw.html#tymethod.from_raw\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.as_raw\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-de3bc13b25639774/out/xproto.rs.html#130-132\">source</a><a href=\"#method.as_raw\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"xcb/trait.Raw.html#tymethod.as_raw\" class=\"fn\">as_raw</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.pointer.html\">*mut </a><a class=\"struct\" href=\"xcb/ffi/struct.xcb_generic_error_t.html\" title=\"struct xcb::ffi::xcb_generic_error_t\">xcb_generic_error_t</a></h4></section></summary><div class='docblock'>Obtain the raw pointer representation</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_raw\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/base.rs.html#67-71\">source</a><a href=\"#method.into_raw\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"xcb/trait.Raw.html#method.into_raw\" class=\"fn\">into_raw</a>(self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.pointer.html\">*mut T</a></h4></section></summary><div class='docblock'>Convert self into a raw pointer <a href=\"xcb/trait.Raw.html#method.into_raw\">Read more</a></div></details></div></details>","Raw<xcb_generic_error_t>","xcb::xproto::WindowError","xcb::xproto::PixmapError","xcb::xproto::AtomError","xcb::xproto::CursorError","xcb::xproto::FontError","xcb::xproto::DrawableError","xcb::xproto::ColormapError","xcb::xproto::GContextError","xcb::xproto::IdChoiceError","xcb::shm::BadSegError"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-ValueError\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-de3bc13b25639774/out/xproto.rs.html#199-211\">source</a><a href=\"#impl-Debug-for-ValueError\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"xcb/x/struct.ValueError.html\" title=\"struct xcb::x::ValueError\">ValueError</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-de3bc13b25639774/out/xproto.rs.html#200-210\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.1/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.1/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.77.1/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.77.1/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","xcb::xproto::WindowError","xcb::xproto::PixmapError","xcb::xproto::AtomError","xcb::xproto::CursorError","xcb::xproto::FontError","xcb::xproto::DrawableError","xcb::xproto::ColormapError","xcb::xproto::GContextError","xcb::xproto::IdChoiceError","xcb::shm::BadSegError"],["<section id=\"impl-Send-for-ValueError\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-de3bc13b25639774/out/xproto.rs.html#221\">source</a><a href=\"#impl-Send-for-ValueError\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"xcb/x/struct.ValueError.html\" title=\"struct xcb::x::ValueError\">ValueError</a></h3></section>","Send","xcb::xproto::WindowError","xcb::xproto::PixmapError","xcb::xproto::AtomError","xcb::xproto::CursorError","xcb::xproto::FontError","xcb::xproto::DrawableError","xcb::xproto::ColormapError","xcb::xproto::GContextError","xcb::xproto::IdChoiceError","xcb::shm::BadSegError"],["<section id=\"impl-Sync-for-ValueError\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-de3bc13b25639774/out/xproto.rs.html#222\">source</a><a href=\"#impl-Sync-for-ValueError\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"xcb/x/struct.ValueError.html\" title=\"struct xcb::x::ValueError\">ValueError</a></h3></section>","Sync","xcb::xproto::WindowError","xcb::xproto::PixmapError","xcb::xproto::AtomError","xcb::xproto::CursorError","xcb::xproto::FontError","xcb::xproto::DrawableError","xcb::xproto::ColormapError","xcb::xproto::GContextError","xcb::xproto::IdChoiceError","xcb::shm::BadSegError"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BaseError-for-ValueError\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-de3bc13b25639774/out/xproto.rs.html#135-139\">source</a><a href=\"#impl-BaseError-for-ValueError\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"xcb/trait.BaseError.html\" title=\"trait xcb::BaseError\">BaseError</a> for <a class=\"struct\" href=\"xcb/x/struct.ValueError.html\" title=\"struct xcb::x::ValueError\">ValueError</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedconstant.EXTENSION\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-de3bc13b25639774/out/xproto.rs.html#136\">source</a><a href=\"#associatedconstant.EXTENSION\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"xcb/trait.BaseError.html#associatedconstant.EXTENSION\" class=\"constant\">EXTENSION</a>: <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.1/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"enum\" href=\"xcb/enum.Extension.html\" title=\"enum xcb::Extension\">Extension</a>&gt; = None</h4></section></summary><div class='docblock'>The extension associated to this error, or <code>None</code> for the main protocol</div></details><details class=\"toggle\" open><summary><section id=\"associatedconstant.NUMBER\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-de3bc13b25639774/out/xproto.rs.html#138\">source</a><a href=\"#associatedconstant.NUMBER\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"xcb/trait.BaseError.html#associatedconstant.NUMBER\" class=\"constant\">NUMBER</a>: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.1/std/primitive.u32.html\">u32</a> = 2u32</h4></section></summary><div class='docblock'>The number associated to this error</div></details></div></details>","BaseError","xcb::xproto::WindowError","xcb::xproto::PixmapError","xcb::xproto::AtomError","xcb::xproto::CursorError","xcb::xproto::FontError","xcb::xproto::DrawableError","xcb::xproto::ColormapError","xcb::xproto::GContextError","xcb::xproto::IdChoiceError","xcb::shm::BadSegError"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Drop-for-ValueError\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-de3bc13b25639774/out/xproto.rs.html#213-219\">source</a><a href=\"#impl-Drop-for-ValueError\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.1/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"xcb/x/struct.ValueError.html\" title=\"struct xcb::x::ValueError\">ValueError</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.drop\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-de3bc13b25639774/out/xproto.rs.html#214-218\">source</a><a href=\"#method.drop\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.1/core/ops/drop/trait.Drop.html#tymethod.drop\" class=\"fn\">drop</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Executes the destructor for this type. <a href=\"https://doc.rust-lang.org/1.77.1/core/ops/drop/trait.Drop.html#tymethod.drop\">Read more</a></div></details></div></details>","Drop","xcb::xproto::WindowError","xcb::xproto::PixmapError","xcb::xproto::AtomError","xcb::xproto::CursorError","xcb::xproto::FontError","xcb::xproto::DrawableError","xcb::xproto::ColormapError","xcb::xproto::GContextError","xcb::xproto::IdChoiceError","xcb::shm::BadSegError"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()