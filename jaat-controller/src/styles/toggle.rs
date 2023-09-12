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
