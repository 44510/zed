// import { text, background } from "./components"
// import { toggleable } from "../element"
// import { useTheme } from "src/theme"

// export default function command_palette(): any {
//     const theme = useTheme()

//     const key = toggleable({
//         base: {
//             text: text(theme.highest, "mono", "variant", "default", {
//                 size: "xs",
//             }),
//             corner_radius: 2,
//             background: background(theme.highest, "on"),
//             padding: {
//                 top: 1,
//                 bottom: 1,
//                 left: 6,
//                 right: 6,
//             },
//             margin: {
//                 top: 1,
//                 bottom: 1,
//                 left: 2,
//             },
//         },
//         state: {
//             active: {
//                 text: text(theme.highest, "mono", "on", "default", {
//                     size: "xs",
//                 }),
//                 background: with_opacity(background(theme.highest, "on"), 0.2),
//             },
//         },
//     })

//     return {
//         keystroke_spacing: 8,
//         // TODO: This should be a Toggle<ContainedText> on the rust side so we don't have to do this
//         key: {
//             inactive: { ...key.inactive },
//             active: key.active,
//         },
//     }
// }
