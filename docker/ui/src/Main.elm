module Main exposing (main)

import Array exposing (..)
import Browser
import Debug exposing (..)
import Dict exposing (..)
import Html exposing (..)
import Theme exposing (..)
import W.Badge
import W.Button
import W.Container
import W.InputInt exposing (..)
import W.Styles
import W.Text


main : Program () Model Msg
main =
    Browser.sandbox
        { init = init
        , view = view
        , update = update
        }


type State
    = Entering
    | Solving
    | Solved


constants =
    { validSmallNumbers = List.range 1 10
    , validLargeNumbers = [ 25, 50, 75, 100 ]
    , validDefinedGoals = List.range 100 999
    }


type alias Model =
    { numberInputs : Array W.InputInt.Value
    , goalInput : W.InputInt.Value
    , state : State
    , solution : Maybe String
    }


isValidNumbers : Array W.InputInt.Value -> Bool
isValidNumbers values =
    let
        validValues =
            Array.toList values
                |> List.filter isValidNumber
                |> List.map W.InputInt.toInt
                |> List.filterMap (\v -> v)

        validSmallCounts =
            validValues
                |> List.filter (\n -> List.member n (List.range 1 10))
                |> groupNumbers
                |> Dict.values
                |> List.filter (\n -> 0 < n && n <= 2)
                |> List.sum

        validLargeCounts =
            validValues
                |> List.filter (\n -> List.member n [ 25, 50, 75, 100 ])
                |> groupNumbers
                |> Dict.values
                |> List.filter (\n -> 0 <= 0 && n <= 1)
                |> List.sum
    in
    List.length validValues == 6 && (validSmallCounts + validLargeCounts == 6)


isValidNumber : W.InputInt.Value -> Bool
isValidNumber value =
    case W.InputInt.toInt value of
        Just n ->
            List.member n (constants.validSmallNumbers ++ constants.validLargeNumbers)

        Nothing ->
            False


groupNumbers : List Int -> Dict Int Int
groupNumbers numbers =
    numbers
        |> List.foldr
            (\tag carry ->
                Dict.update
                    tag
                    (\maybeExistingCount ->
                        case maybeExistingCount of
                            Just existingCount ->
                                Just (existingCount + 1)

                            Nothing ->
                                Just 1
                    )
                    carry
            )
            Dict.empty


isValidGoal : W.InputInt.Value -> Bool
isValidGoal value =
    if String.isEmpty (W.InputInt.toString value) then
        True

    else
        case W.InputInt.toInt value of
            Just n ->
                List.member n constants.validDefinedGoals

            Nothing ->
                False


type Msg
    = NumberUpdated Int W.InputInt.Value
    | GoalUpdated W.InputInt.Value
    | Solve
    | SolutionFound String
    | Reset


init : Model
init =
    { numberInputs = Array.repeat 6 (W.InputInt.init Nothing)
    , goalInput = W.InputInt.init Nothing
    , state = Entering
    , solution = Nothing
    }


view : Model -> Html Msg
view model =
    div []
        [ W.Styles.globalStyles
        , W.Styles.baseTheme
        , W.Container.view
            [ W.Container.vertical
            , W.Container.spaceAround
            , W.Container.fill
            , W.Container.pad_4
            , W.Container.alignCenterX
            ]
            [ header
            , content model
            , footer
            ]
        ]


header : Html msg
header =
    W.Text.view [ W.Text.extraLarge ] [ text "Letters and Numbers" ]


content : Model -> Html Msg
content model =
    W.Container.view []
        [ numberInputs model.numberInputs model
        , goalInput model.goalInput model
        , commands model
        , solution model
        ]


numberInputs : Array W.InputInt.Value -> Model -> Html Msg
numberInputs numbers model =
    let
        indexedInputs =
            Array.toIndexedList numbers
    in
    W.Badge.view
        [ W.Badge.small, W.Badge.warning ]
        { content = warningBadge (isValidNumbers numbers)
        , children =
            [ W.Container.view
                [ W.Container.horizontal
                , W.Container.spaceAround
                ]
                (List.map (\( i, input ) -> numberInput i input model) indexedInputs)
            ]
        }


warningBadge : Bool -> Maybe (List (Html Msg))
warningBadge ok =
    if ok then
        Nothing

    else
        Just [ text "⚠" ]


numberInput : Int -> W.InputInt.Value -> Model -> Html Msg
numberInput i n model =
    W.Badge.view
        [ W.Badge.small, W.Badge.warning ]
        { content = warningBadge (isValidNumber n)
        , children =
            [ W.InputInt.view
                [ W.InputInt.disabled (model.state /= Entering)
                ]
                { onInput = NumberUpdated i
                , value = n
                }
            ]
        }


goalInput : W.InputInt.Value -> Model -> Html Msg
goalInput goal model =
    W.Badge.view
        [ W.Badge.warning ]
        { content = warningBadge (isValidGoal goal)
        , children =
            [ W.InputInt.view
                [ W.InputInt.min 100
                , W.InputInt.max 999
                , W.InputInt.placeholder "- Auto -"
                , W.InputInt.disabled (model.state /= Entering)
                ]
                { onInput = GoalUpdated
                , value = goal
                }
            ]
        }


commands : Model -> Html Msg
commands model =
    let
        canFindSolution =
            model.state == Entering && isValidNumbers model.numberInputs && isValidGoal model.goalInput

        canReset =
            model.state == Solved
    in
    W.Container.view []
        [ W.Button.view [ W.Button.primary, W.Button.disabled (not canFindSolution) ]
            { label = [ text "Find solution" ]
            , onClick = Solve
            }
        , W.Button.view [ W.Button.primary, W.Button.disabled (not canReset) ]
            { label = [ text "Reset" ]
            , onClick = Reset
            }
        ]


solution : Model -> Html Msg
solution model =
    let
        solutionText =
            case model.solution of
                Just s ->
                    s

                Nothing ->
                    ""
    in
    W.Text.view [ W.Text.large, W.Text.alignCenter ] [ text solutionText ]


footer : Html msg
footer =
    W.Text.view [ W.Text.extraSmall ] [ text "Copyright (c) 2023; Nigel Eke. All rights reserved." ]


update : Msg -> Model -> Model
update msg model =
    case msg of
        NumberUpdated i value ->
            { model | numberInputs = Array.set i value model.numberInputs }

        GoalUpdated value ->
            { model | goalInput = value }

        Solve ->
            { model | state = Solving }

        SolutionFound s ->
            { model | state = Solved, solution = Just s }

        Reset ->
            { model | state = Entering }
