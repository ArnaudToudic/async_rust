import React, { Component } from 'react'

import Block from '@components/molecules/block'

type State = {
  quantity: number
}

export default class List extends Component<{}, State> {
  readonly state: State = {
    quantity: 12
  }

  constructor(props) {
    super(props)

    this.handleChange = this.handleChange.bind(this)
    this.handleDecrement = this.handleDecrement.bind(this)
    this.handleIncrement = this.handleIncrement.bind(this)
  }

  handleChange(e: import('react').ChangeEvent<HTMLInputElement>) {
    this.setState({
      quantity: parseInt(e.target.value) ? parseInt(e.target.value) : 0
    })
  }

  handleDecrement() {
    if (this.state.quantity > 0) {
      this.setState({
        quantity: this.state.quantity - 1
      })
    }
  }

  handleIncrement() {
    this.setState({
      quantity: this.state.quantity + 1
    })
  }

  render() {
    return (
      <Block
        title="test"
        handleChange={this.handleChange}
        handleDecrement={this.handleDecrement}
        handleIncrement={this.handleIncrement}
        quantity={this.state.quantity.toString()}
      />
    )
  }
}
