import chroma from "chroma-js"
import { Meta } from "../common/colorScheme"
import { colorRamp, createColorScheme } from "../common/ramps"
import { metaCommon, name, buildSyntax, Variant } from "./atelier-common"

const variant: Variant = {
    meta: {
        name: `${name} Forest Dark`,
        ...metaCommon,
        url: "https://atelierbram.github.io/syntax-highlighting/atelier-schemes/forest/",
    },
    colors: {
        base00: "#1b1918",
        base01: "#2c2421",
        base02: "#68615e",
        base03: "#766e6b",
        base04: "#9c9491",
        base05: "#a8a19f",
        base06: "#e6e2e0",
        base07: "#f1efee",
        base08: "#f22c40",
        base09: "#df5320",
        base0A: "#c38418",
        base0B: "#7b9726",
        base0C: "#3d97b8",
        base0D: "#407ee7",
        base0E: "#6666ea",
        base0F: "#c33ff3",
    },
}

const syntax = buildSyntax(variant)

const theme = (variant: Variant) => {
    const { meta, colors } = variant

    return createColorScheme(
        meta.name,
        false,
        {
            neutral: chroma.scale([
                colors.base00,
                colors.base01,
                colors.base02,
                colors.base03,
                colors.base04,
                colors.base05,
                colors.base06,
                colors.base07,
            ]),
            red: colorRamp(chroma(colors.base08)),
            orange: colorRamp(chroma(colors.base09)),
            yellow: colorRamp(chroma(colors.base0A)),
            green: colorRamp(chroma(colors.base0B)),
            cyan: colorRamp(chroma(colors.base0C)),
            blue: colorRamp(chroma(colors.base0D)),
            violet: colorRamp(chroma(colors.base0E)),
            magenta: colorRamp(chroma(colors.base0F)),
        },
        syntax
    )
}

export const dark = theme(variant)

export const meta: Meta = variant.meta
