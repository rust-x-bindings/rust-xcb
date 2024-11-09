(function() {
    var type_impls = Object.fromEntries([["xcb",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BaseEvent-for-FocusInEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#1361-1364\">source</a><a href=\"#impl-BaseEvent-for-FocusInEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"xcb/trait.BaseEvent.html\" title=\"trait xcb::BaseEvent\">BaseEvent</a> for <a class=\"struct\" href=\"xcb/x/struct.FocusInEvent.html\" title=\"struct xcb::x::FocusInEvent\">FocusInEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedconstant.EXTENSION\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#1362\">source</a><a href=\"#associatedconstant.EXTENSION\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"xcb/trait.BaseEvent.html#associatedconstant.EXTENSION\" class=\"constant\">EXTENSION</a>: <a class=\"enum\" href=\"https://doc.rust-lang.org/1.82.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"enum\" href=\"xcb/enum.Extension.html\" title=\"enum xcb::Extension\">Extension</a>&gt; = None</h4></section></summary><div class='docblock'>The extension associated to this event, or <code>None</code> for the main protocol</div></details><details class=\"toggle\" open><summary><section id=\"associatedconstant.NUMBER\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#1363\">source</a><a href=\"#associatedconstant.NUMBER\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"xcb/trait.BaseEvent.html#associatedconstant.NUMBER\" class=\"constant\">NUMBER</a>: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.u32.html\">u32</a> = 9u32</h4></section></summary><div class='docblock'>The number associated to this event</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_from_send_event\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/base.rs.html#87-91\">source</a><a href=\"#method.is_from_send_event\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"xcb/trait.BaseEvent.html#method.is_from_send_event\" class=\"fn\">is_from_send_event</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Check whether this event was emitted by the SendEvent request\nSee <code>[crate::x::SendEvent]</code>.</div></details></div></details>","BaseEvent","xcb::xproto::FocusOutEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-FocusInEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#1437-1448\">source</a><a href=\"#impl-Debug-for-FocusInEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"xcb/x/struct.FocusInEvent.html\" title=\"struct xcb::x::FocusInEvent\">FocusInEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#1438-1447\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","xcb::xproto::FocusOutEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Drop-for-FocusInEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#1480-1486\">source</a><a href=\"#impl-Drop-for-FocusInEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"xcb/x/struct.FocusInEvent.html\" title=\"struct xcb::x::FocusInEvent\">FocusInEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.drop\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#1481-1485\">source</a><a href=\"#method.drop\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.82.0/core/ops/drop/trait.Drop.html#tymethod.drop\" class=\"fn\">drop</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Executes the destructor for this type. <a href=\"https://doc.rust-lang.org/1.82.0/core/ops/drop/trait.Drop.html#tymethod.drop\">Read more</a></div></details></div></details>","Drop","xcb::xproto::FocusOutEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-FocusInEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#1366-1435\">source</a><a href=\"#impl-FocusInEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"struct\" href=\"xcb/x/struct.FocusInEvent.html\" title=\"struct xcb::x::FocusInEvent\">FocusInEvent</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#1367-1384\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.FocusInEvent.html#tymethod.new\" class=\"fn\">new</a>(\n    detail: <a class=\"enum\" href=\"xcb/x/enum.NotifyDetail.html\" title=\"enum xcb::x::NotifyDetail\">NotifyDetail</a>,\n    event: <a class=\"struct\" href=\"xcb/x/struct.Window.html\" title=\"struct xcb::x::Window\">Window</a>,\n    mode: <a class=\"enum\" href=\"xcb/x/enum.NotifyMode.html\" title=\"enum xcb::x::NotifyMode\">NotifyMode</a>,\n) -&gt; <a class=\"struct\" href=\"xcb/x/struct.FocusInEvent.html\" title=\"struct xcb::x::FocusInEvent\">FocusInEvent</a></h4></section><section id=\"method.response_type\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#1390-1396\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.FocusInEvent.html#tymethod.response_type\" class=\"fn\">response_type</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.u8.html\">u8</a></h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.detail\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#1399-1406\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.FocusInEvent.html#tymethod.detail\" class=\"fn\">detail</a>(&amp;self) -&gt; <a class=\"enum\" href=\"xcb/x/enum.NotifyDetail.html\" title=\"enum xcb::x::NotifyDetail\">NotifyDetail</a></h4></section></summary><div class=\"docblock\"></div></details><section id=\"method.sequence\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#1408-1414\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.FocusInEvent.html#tymethod.sequence\" class=\"fn\">sequence</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.u16.html\">u16</a></h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.event\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#1418-1424\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.FocusInEvent.html#tymethod.event\" class=\"fn\">event</a>(&amp;self) -&gt; <a class=\"struct\" href=\"xcb/x/struct.Window.html\" title=\"struct xcb::x::Window\">Window</a></h4></section></summary><div class=\"docblock\"><p>The window on which the focus event was generated. This is the window used by\nthe X server to report the event.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.mode\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#1427-1434\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/x/struct.FocusInEvent.html#tymethod.mode\" class=\"fn\">mode</a>(&amp;self) -&gt; <a class=\"enum\" href=\"xcb/x/enum.NotifyMode.html\" title=\"enum xcb::x::NotifyMode\">NotifyMode</a></h4></section></summary><div class=\"docblock\"></div></details></div></details>",0,"xcb::xproto::FocusOutEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Raw%3Cxcb_generic_event_t%3E-for-FocusInEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#1351-1359\">source</a><a href=\"#impl-Raw%3Cxcb_generic_event_t%3E-for-FocusInEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"xcb/trait.Raw.html\" title=\"trait xcb::Raw\">Raw</a>&lt;<a class=\"struct\" href=\"xcb/ffi/struct.xcb_generic_event_t.html\" title=\"struct xcb::ffi::xcb_generic_event_t\">xcb_generic_event_t</a>&gt; for <a class=\"struct\" href=\"xcb/x/struct.FocusInEvent.html\" title=\"struct xcb::x::FocusInEvent\">FocusInEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_raw\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#1352-1354\">source</a><a href=\"#method.from_raw\" class=\"anchor\">§</a><h4 class=\"code-header\">unsafe fn <a href=\"xcb/trait.Raw.html#tymethod.from_raw\" class=\"fn\">from_raw</a>(raw: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.pointer.html\">*mut </a><a class=\"struct\" href=\"xcb/ffi/struct.xcb_generic_event_t.html\" title=\"struct xcb::ffi::xcb_generic_event_t\">xcb_generic_event_t</a>) -&gt; Self</h4></section></summary><div class='docblock'>Build <code>Self</code> from a raw pointer <a href=\"xcb/trait.Raw.html#tymethod.from_raw\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.as_raw\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#1356-1358\">source</a><a href=\"#method.as_raw\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"xcb/trait.Raw.html#tymethod.as_raw\" class=\"fn\">as_raw</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.pointer.html\">*mut </a><a class=\"struct\" href=\"xcb/ffi/struct.xcb_generic_event_t.html\" title=\"struct xcb::ffi::xcb_generic_event_t\">xcb_generic_event_t</a></h4></section></summary><div class='docblock'>Obtain the raw pointer representation</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_raw\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/base.rs.html#67-71\">source</a><a href=\"#method.into_raw\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"xcb/trait.Raw.html#method.into_raw\" class=\"fn\">into_raw</a>(self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.pointer.html\">*mut T</a></h4></section></summary><div class='docblock'>Convert self into a raw pointer <a href=\"xcb/trait.Raw.html#method.into_raw\">Read more</a></div></details></div></details>","Raw<xcb_generic_event_t>","xcb::xproto::FocusOutEvent"],["<section id=\"impl-Send-for-FocusInEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#1488\">source</a><a href=\"#impl-Send-for-FocusInEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"xcb/x/struct.FocusInEvent.html\" title=\"struct xcb::x::FocusInEvent\">FocusInEvent</a></h3></section>","Send","xcb::xproto::FocusOutEvent"],["<section id=\"impl-Sync-for-FocusInEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-f41af313ebba36d7/out/xproto.rs.html#1489\">source</a><a href=\"#impl-Sync-for-FocusInEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"xcb/x/struct.FocusInEvent.html\" title=\"struct xcb::x::FocusInEvent\">FocusInEvent</a></h3></section>","Sync","xcb::xproto::FocusOutEvent"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[14455]}