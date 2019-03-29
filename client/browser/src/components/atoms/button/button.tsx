import React from 'react'

import './style.scss'

export const ButtonType = {
  BUTTON: 'button',
  RESET: 'reset',
  SUBMIT: 'submit'
}

type Props = {
  onClick: (e: import('react').MouseEvent) => void
  children: import('react').ReactChild
} & typeof defaultProps

const defaultProps = {
  type: ButtonType.BUTTON,
  disabled: false
}

const Button = (props: Props) => (
  <button
    type={props.type}
    onClick={props.onClick}
    disabled={props.disabled}
    className="button"
  >
    {props.children}
  </button>
)

Button.defaultProps = defaultProps

export default Button
