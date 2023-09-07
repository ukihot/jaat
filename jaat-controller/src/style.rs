pub struct Style {}

impl Style {
    pub const GRID_STYLES: &str = r#"
        .grid_container {
            display:grid;
            grid-gap:10px 10%;
        }
    "#;

    pub const TEXTFORM_STYLES: &str = r#"
        .player_info {
            height: 2rem;
            width: 38%;
            padding: 0 16px;
            border-radius: 4px;
            border: none;
            box-shadow: 0 0 0 2px #ccc inset;
        }
        .player_info:focus {
            outline: 0;
            box-shadow: 0 0 0 2px rgb(33, 150, 243) inset;
        }
    "#;

    pub const TOGGLE_STYLES: &str = r#"
        .switch_label {
            width: 50px;
            position: relative;
            display: inline-block;
        }
        .switch_content {
            display: block;
            cursor: pointer;
            position: relative;
            border-radius: 30px;
            height: 31px;
            overflow: hidden;
        }
        .switch_content:before {
            content: "";
            display: block;
            position: absolute;
            width: calc(100% - 3px);
            height: calc(100% - 3px);
            top: 0;
            left: 0;
            border: 1.5px solid #E5E5EA;
            border-radius: 30px;
            background-color: #fff;
        }
        .switch_content:after {
            content: "";
            display: block;
            position: absolute;
            background-color: transparent;
            width: 0;
            height: 0;
            top: 50%;
            left: 50%;
            border-radius: 30px;
            -webkit-transition: all .5s;
            -moz-transition: all .5s;
            -ms-transition: all .5s;
            -o-transition: all .5s;
            transition: all .5s;
        }
        .switch_input {
            display: none;
        }
        .switch_circle {
            display: block;
            top: 2px;
            left: 2px;
            position: absolute;
            -webkit-box-shadow: 0 2px 6px #999;
            box-shadow: 0 2px 6px #999;
            width: 27px;
            height: 27px;
            -webkit-border-radius: 20px;
            border-radius: 20px;
            background-color: #fff;
            -webkit-transition: all .5s;
            -moz-transition: all .5s;
            -ms-transition: all .5s;
            -o-transition: all .5s;
            transition: all .5s;
        }
        .switch_input:checked ~ .switch_circle {
            left: 21px;
        }
        .switch_input:checked ~ .switch_content:after {
            background-color: #00c4cc;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
        }
    "#;

    pub const RADIO_STYLES: &str = r#"
        .first_atack {
            display: flex;
            margin-bottom: 0.5rem;
            padding: 1rem 0.5rem;
        }

        .radio_area input[type=radio] {
            position: absolute;
            -webkit-appearance: none;
            -moz-appearance: none;
            appearance: none;
        }

        .radio_area label {
            cursor: pointer;
            padding: 1rem;
            color: #fff;
            background-color: #6a94b7;
            transition: .5s;
        }
        .radio_area:first-child label {
            border-top-left-radius: 0.5rem;
            border-bottom-left-radius: 0.5rem;
        }
        .radio_area:last-child label {
            border-top-right-radius: 0.5rem;
            border-bottom-right-radius: 0.5rem;
        }
        .radio_area input[type=radio]:checked+label {
            background-color: #3079b5;
        }
    "#;

    pub const RESET: &str = r#"
    *:where(:not(html, iframe, canvas, img, svg, video, audio):not(svg *, symbol *)) {
            all: unset;
            display: revert;
        }
        *,
        *::before,
        *::after {
            box-sizing: border-box;
        }
        a, button {
            cursor: revert;
        }
        ol, ul, menu {
            list-style: none;
        }
        img {
            max-inline-size: 100%;
            max-block-size: 100%;
        }
        table {
            border-collapse: collapse;
        }
        input, textarea {
            -webkit-user-select: auto;
        }
        textarea {
            white-space: revert;
        }
        meter {
            -webkit-appearance: revert;
            appearance: revert;
        }
        :where(pre) {
            all: revert;
        }
        ::placeholder {
            color: unset;
        }
        ::marker {
            content: initial;
        }
        :where([hidden]) {
            display: none;
        }
        :where([contenteditable]:not([contenteditable="false"])) {
            -moz-user-modify: read-write;
            -webkit-user-modify: read-write;
            overflow-wrap: break-word;
            -webkit-line-break: after-white-space;
            -webkit-user-select: auto;
        }
        :where([draggable="true"]) {
            -webkit-user-drag: element;
        }
        :where(dialog:modal) {
            all: revert;
        }
    "#;
}
