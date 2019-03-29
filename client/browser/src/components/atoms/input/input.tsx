import React from 'react'

import './style.scss'

type Props = {
  onChange: (e: import('react').ChangeEvent<HTMLInputElement>) => void
  pattern?: string
} & typeof defaultProps

const defaultProps = {
  value: ''
}

const Input = (props: Props) => (
  <input
    className="input"
    onChange={props.onChange}
    pattern={props.pattern}
    value={props.value}
  />
)

Input.defaultProps = defaultProps

export default Input
