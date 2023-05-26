import chroma from "chroma-js"
import { Meta } from "../common/colorScheme"
import { colorRamp, createColorScheme } from "../common/ramps"
import { metaCommon, name, buildSyntax, Variant } from "./atelier-common"

const variant: Variant = {
    meta: {
        name: `${name} Lakeside Light`,
        ...metaCommon,
        url: "https://atelierbram.github.io/syntax-highlighting/atelier-schemes/lakeside/",
    },
    colors: {
        base00: "#ebf8ff",
        base01: "#c1e4f6",
        base02: "#7ea2b4",
        base03: "#7195a8",
        base04: "#5a7b8c",
        base05: "#516d7b",
        base06: "#1f292e",
        base07: "#161b1d",
        base08: "#d22d72",
        base09: "#935c25",
        base0A: "#8a8a0f",
        base0B: "#568c3b",
        base0C: "#2d8f6f",
        base0D: "#257fad",
        base0E: "#6b6bb8",
        base0F: "#b72dd2",
    },
}

const syntax = buildSyntax(variant)

const theme = (variant: Variant) => {
    const { meta, colors } = variant

    return createColorScheme(
        meta.name,
        true,
        {
            neutral: chroma.scale(
                [
                    colors.base00,
                    colors.base01,
                    colors.base02,
                    colors.base03,
                    colors.base04,
                    colors.base05,
                    colors.base06,
                    colors.base07,
                ].reverse()
            ),
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
