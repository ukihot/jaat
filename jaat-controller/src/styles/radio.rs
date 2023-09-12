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
