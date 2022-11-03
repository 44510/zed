import { Scale } from "chroma-js";
import { Syntax } from "./theme";

export interface ColorScheme {
  name: string;
  isLight: boolean;

  lowest: Layer;
  middle: Layer;
  highest: Layer;

  ramps: RampSet;

  popoverShadow: Shadow;
  modalShadow: Shadow;

  players: Players;
  syntax: Syntax;
}

export interface Player {
  cursor: string;
  selection: string;
}

export interface Players {
  "0": Player;
  "1": Player;
  "2": Player;
  "3": Player;
  "4": Player;
  "5": Player;
  "6": Player;
  "7": Player;
}

export interface Shadow {
  blur: number;
  color: string;
  offset: number[];
}

export type StyleSets = keyof Layer;
export interface Layer {
  base: StyleSet;
  variant: StyleSet;
  on: StyleSet;
  accent: StyleSet;
  positive: StyleSet;
  warning: StyleSet;
  negative: StyleSet;
}

export interface RampSet {
  neutral: Scale;
  red: Scale;
  orange: Scale;
  yellow: Scale;
  green: Scale;
  cyan: Scale;
  blue: Scale;
  violet: Scale;
  magenta: Scale;
}

export type Styles = keyof StyleSet;
export interface StyleSet {
  default: Style;
  active: Style;
  disabled: Style;
  hovered: Style;
  pressed: Style;
  inverted: Style;
}

export interface Style {
  background: string;
  border: string;
  foreground: string;
}
