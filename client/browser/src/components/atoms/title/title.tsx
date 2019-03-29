import React from 'react'

import './style.scss'

type Props = {
  children: import('react').ReactChild
}

const Title = (props: Props) => <h1 className="title">{props.children}</h1>

export default Title
