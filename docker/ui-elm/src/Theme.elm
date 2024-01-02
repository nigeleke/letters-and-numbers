module Theme exposing (..)

-- import Html.Attributes exposing (placeholder)


color =
    { theme = "#546F82"
    , themeL5 = "#F4F6F8"
    , themeL4 = "#DBE2E8"
    , themeL3 = "#B7C6D0"
    , themeL2 = "#94A9B9"
    , themeL1 = "#708DA2"
    , themeD1 = "#4C6475"
    , themeD2 = "#445968"
    , themeD3 = "#3B4E5B"
    , themeD4 = "#33434E"
    , themeD5 = "#2A3741"
    , themeLight = "#F4F6F8"
    , themeDark = "#2A3741"
    }


attrs =
    { appBackground = String.join "" [ "linear-gradient(30deg,.", color.themeL1, ",", color.themeD2, ")" ]

    -- , inputText = [ Border.rounded 15, Font.color color.themeD4 ]
    -- , placeholder = [ Font.color color.themeL3 ]
    }



-- -- , actionTheme = color.themeDark
-- -- , textTheme = color.themeL5
-- -- , borderTheme = color.themeL5
-- -- , hoverTheme = color.themeL5
-- -- , hoverTextTheme = color.theme
-- -- , hoverBorderTheme = color.theme
-- -- }
