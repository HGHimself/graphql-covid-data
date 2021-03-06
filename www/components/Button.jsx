import React from "react";
import { css } from "@emotion/css";

import { darkOrLight } from "../utils/color-tools";
import theme from "../theme/";

export default function Button(props) {
  const { type, ...other } = props;

  const color = theme.colors[type];

  const button = css`
    color: ${darkOrLight(color)};
    background-color: ${color};
    border-radius: 0px;
    border: none;
    text-transform: uppercase;
    cursor: pointer;
  `;

  return (
    <button className={button} {...other}>
      {props.children}
    </button>
  );
}
