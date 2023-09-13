pub const GRID_STYLES: &str = r#"
        .grid_form{
            display: grid;
            place-items: center;
            grid-template-rows: repeat(9, 1fr);
            grid-template-columns: repeat(2, 1fr);
            gap: 1rem;
            grid-auto-flow: column;

            .grid_form_details {
                width: 62%;
                display: grid;
                justify-content: center;
                align-items: center;
                grid-template-columns: repeat(12, 1fr);

                .uniform {
                    grid-area: 1 /1 /3 /4;
                }

                .name {
                    grid-area: 1 /4 /2 /13;
                }

                .gender {
                    grid-area: 2 /4 /3 /7;
                }

                .height {
                    grid-area: 2 /7 /3 /10;
                }

                .weight {
                    grid-area: 2 /10 /3 /13;
                }
            }
        }
    "#;
