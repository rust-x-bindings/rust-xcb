(function() {var type_impls = {
"xcb":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-DeviceFocusInEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xinput.rs.html#911-1002\">source</a><a href=\"#impl-DeviceFocusInEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"struct\" href=\"xcb/xinput/struct.DeviceFocusInEvent.html\" title=\"struct xcb::xinput::DeviceFocusInEvent\">DeviceFocusInEvent</a></h3></section></summary><div class=\"impl-items\"><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xinput.rs.html#912-939\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.DeviceFocusInEvent.html#tymethod.new\" class=\"fn\">new</a>(\n    event_base: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u8.html\">u8</a>,\n    detail: <a class=\"enum\" href=\"xcb/x/enum.NotifyDetail.html\" title=\"enum xcb::x::NotifyDetail\">NotifyDetail</a>,\n    time: <a class=\"type\" href=\"xcb/x/type.Timestamp.html\" title=\"type xcb::x::Timestamp\">Timestamp</a>,\n    window: <a class=\"struct\" href=\"xcb/x/struct.Window.html\" title=\"struct xcb::x::Window\">Window</a>,\n    mode: <a class=\"enum\" href=\"xcb/x/enum.NotifyMode.html\" title=\"enum xcb::x::NotifyMode\">NotifyMode</a>,\n    device_id: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u8.html\">u8</a>\n) -&gt; <a class=\"struct\" href=\"xcb/xinput/struct.DeviceFocusInEvent.html\" title=\"struct xcb::xinput::DeviceFocusInEvent\">DeviceFocusInEvent</a></h4></section><section id=\"method.response_type\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xinput.rs.html#945-951\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.DeviceFocusInEvent.html#tymethod.response_type\" class=\"fn\">response_type</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u8.html\">u8</a></h4></section><section id=\"method.detail\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xinput.rs.html#953-960\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.DeviceFocusInEvent.html#tymethod.detail\" class=\"fn\">detail</a>(&amp;self) -&gt; <a class=\"enum\" href=\"xcb/x/enum.NotifyDetail.html\" title=\"enum xcb::x::NotifyDetail\">NotifyDetail</a></h4></section><section id=\"method.sequence\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xinput.rs.html#962-968\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.DeviceFocusInEvent.html#tymethod.sequence\" class=\"fn\">sequence</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u16.html\">u16</a></h4></section><section id=\"method.time\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xinput.rs.html#970-976\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.DeviceFocusInEvent.html#tymethod.time\" class=\"fn\">time</a>(&amp;self) -&gt; <a class=\"type\" href=\"xcb/x/type.Timestamp.html\" title=\"type xcb::x::Timestamp\">Timestamp</a></h4></section><section id=\"method.window\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xinput.rs.html#978-984\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.DeviceFocusInEvent.html#tymethod.window\" class=\"fn\">window</a>(&amp;self) -&gt; <a class=\"struct\" href=\"xcb/x/struct.Window.html\" title=\"struct xcb::x::Window\">Window</a></h4></section><section id=\"method.mode\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xinput.rs.html#986-993\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.DeviceFocusInEvent.html#tymethod.mode\" class=\"fn\">mode</a>(&amp;self) -&gt; <a class=\"enum\" href=\"xcb/x/enum.NotifyMode.html\" title=\"enum xcb::x::NotifyMode\">NotifyMode</a></h4></section><section id=\"method.device_id\" class=\"method\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xinput.rs.html#995-1001\">source</a><h4 class=\"code-header\">pub fn <a href=\"xcb/xinput/struct.DeviceFocusInEvent.html#tymethod.device_id\" class=\"fn\">device_id</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u8.html\">u8</a></h4></section></div></details>",0,"xcb::xinput::DeviceFocusOutEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BaseEvent-for-DeviceFocusInEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xinput.rs.html#906-909\">source</a><a href=\"#impl-BaseEvent-for-DeviceFocusInEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"xcb/trait.BaseEvent.html\" title=\"trait xcb::BaseEvent\">BaseEvent</a> for <a class=\"struct\" href=\"xcb/xinput/struct.DeviceFocusInEvent.html\" title=\"struct xcb::xinput::DeviceFocusInEvent\">DeviceFocusInEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedconstant.EXTENSION\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xinput.rs.html#907\">source</a><a href=\"#associatedconstant.EXTENSION\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"xcb/trait.BaseEvent.html#associatedconstant.EXTENSION\" class=\"constant\">EXTENSION</a>: <a class=\"enum\" href=\"https://doc.rust-lang.org/1.75.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"enum\" href=\"xcb/enum.Extension.html\" title=\"enum xcb::Extension\">Extension</a>&gt; = _</h4></section></summary><div class='docblock'>The extension associated to this event, or <code>None</code> for the main protocol</div></details><details class=\"toggle\" open><summary><section id=\"associatedconstant.NUMBER\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xinput.rs.html#908\">source</a><a href=\"#associatedconstant.NUMBER\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"xcb/trait.BaseEvent.html#associatedconstant.NUMBER\" class=\"constant\">NUMBER</a>: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u32.html\">u32</a> = 6u32</h4></section></summary><div class='docblock'>The number associated to this event</div></details></div></details>","BaseEvent","xcb::xinput::DeviceFocusOutEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Raw%3Cxcb_generic_event_t%3E-for-DeviceFocusInEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xinput.rs.html#896-904\">source</a><a href=\"#impl-Raw%3Cxcb_generic_event_t%3E-for-DeviceFocusInEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"xcb/trait.Raw.html\" title=\"trait xcb::Raw\">Raw</a>&lt;<a class=\"struct\" href=\"xcb/ffi/struct.xcb_generic_event_t.html\" title=\"struct xcb::ffi::xcb_generic_event_t\">xcb_generic_event_t</a>&gt; for <a class=\"struct\" href=\"xcb/xinput/struct.DeviceFocusInEvent.html\" title=\"struct xcb::xinput::DeviceFocusInEvent\">DeviceFocusInEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_raw\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xinput.rs.html#897-899\">source</a><a href=\"#method.from_raw\" class=\"anchor\">§</a><h4 class=\"code-header\">unsafe fn <a href=\"xcb/trait.Raw.html#tymethod.from_raw\" class=\"fn\">from_raw</a>(raw: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.pointer.html\">*mut </a><a class=\"struct\" href=\"xcb/ffi/struct.xcb_generic_event_t.html\" title=\"struct xcb::ffi::xcb_generic_event_t\">xcb_generic_event_t</a>) -&gt; Self</h4></section></summary><div class='docblock'>Build <code>Self</code> from a raw pointer <a href=\"xcb/trait.Raw.html#tymethod.from_raw\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.as_raw\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xinput.rs.html#901-903\">source</a><a href=\"#method.as_raw\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"xcb/trait.Raw.html#tymethod.as_raw\" class=\"fn\">as_raw</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.pointer.html\">*mut </a><a class=\"struct\" href=\"xcb/ffi/struct.xcb_generic_event_t.html\" title=\"struct xcb::ffi::xcb_generic_event_t\">xcb_generic_event_t</a></h4></section></summary><div class='docblock'>Obtain the raw pointer representation</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_raw\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/base.rs.html#67-71\">source</a><a href=\"#method.into_raw\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"xcb/trait.Raw.html#method.into_raw\" class=\"fn\">into_raw</a>(self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.pointer.html\">*mut T</a></h4></section></summary><div class='docblock'>Convert self into a raw pointer <a href=\"xcb/trait.Raw.html#method.into_raw\">Read more</a></div></details></div></details>","Raw<xcb_generic_event_t>","xcb::xinput::DeviceFocusOutEvent"],["<section id=\"impl-Sync-for-DeviceFocusInEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xinput.rs.html#1058\">source</a><a href=\"#impl-Sync-for-DeviceFocusInEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"xcb/xinput/struct.DeviceFocusInEvent.html\" title=\"struct xcb::xinput::DeviceFocusInEvent\">DeviceFocusInEvent</a></h3></section>","Sync","xcb::xinput::DeviceFocusOutEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-DeviceFocusInEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xinput.rs.html#1004-1017\">source</a><a href=\"#impl-Debug-for-DeviceFocusInEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"xcb/xinput/struct.DeviceFocusInEvent.html\" title=\"struct xcb::xinput::DeviceFocusInEvent\">DeviceFocusInEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xinput.rs.html#1005-1016\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.75.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.75.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.75.0/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.75.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","xcb::xinput::DeviceFocusOutEvent"],["<section id=\"impl-Send-for-DeviceFocusInEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xinput.rs.html#1057\">source</a><a href=\"#impl-Send-for-DeviceFocusInEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"xcb/xinput/struct.DeviceFocusInEvent.html\" title=\"struct xcb::xinput::DeviceFocusInEvent\">DeviceFocusInEvent</a></h3></section>","Send","xcb::xinput::DeviceFocusOutEvent"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Drop-for-DeviceFocusInEvent\" class=\"impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xinput.rs.html#1049-1055\">source</a><a href=\"#impl-Drop-for-DeviceFocusInEvent\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"xcb/xinput/struct.DeviceFocusInEvent.html\" title=\"struct xcb::xinput::DeviceFocusInEvent\">DeviceFocusInEvent</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.drop\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/xcb/home/runner/work/rust-xcb/rust-xcb/target/debug/build/xcb-44f85533b0436566/out/xinput.rs.html#1050-1054\">source</a><a href=\"#method.drop\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.75.0/core/ops/drop/trait.Drop.html#tymethod.drop\" class=\"fn\">drop</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Executes the destructor for this type. <a href=\"https://doc.rust-lang.org/1.75.0/core/ops/drop/trait.Drop.html#tymethod.drop\">Read more</a></div></details></div></details>","Drop","xcb::xinput::DeviceFocusOutEvent"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()