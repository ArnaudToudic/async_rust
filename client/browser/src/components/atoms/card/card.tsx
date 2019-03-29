import React from 'react'

import './style.scss'

type Props = {
  children?: import('react').ReactChild
}

const Card = (props: Props) => <div className="card">{props.children}</div>

export default Card
