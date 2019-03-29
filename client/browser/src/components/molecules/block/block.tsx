import React from 'react'

import Button from '@components/atoms/button'
import Card from '@components/atoms/card'
import Input from '@components/atoms/input'
import Title from '@components/atoms/title'

type Props = {
  title: string
  quantity: string
  handleChange: (e: import('react').ChangeEvent<HTMLInputElement>) => void
  handleDecrement: (e: import('react').MouseEvent) => void
  handleIncrement: (e: import('react').MouseEvent) => void
}

const Block = (props: Props) => (
  <Card>
    <>
      <Title>{props.title}</Title>
      <Button onClick={props.handleDecrement}>-</Button>
      <Input
        value={props.quantity}
        onChange={props.handleChange}
        pattern="^\d*$"
      />
      <Button onClick={props.handleIncrement}>+</Button>
    </>
  </Card>
)

export default Block
