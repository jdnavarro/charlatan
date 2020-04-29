import React from "react"

import Button from "@material-ui/core/Button"
import Container from "@material-ui/core/Container"
import Drawer from "@material-ui/core/Drawer"

export default function TemporaryDrawer() {
  const [state, setState] = React.useState({
    left: false,
  })

  const toggleDrawer = open => event => {
    if (
      event.type === "keydown" &&
      (event.key === "Tab" || event.key === "Shift")
    ) {
      return
    }

    setState({ ...state, left: open })
  }

  return (
    <Container>
      <Button variant="contained" color="primary" onClick={toggleDrawer(true)}>
        Click me
      </Button>
      <Drawer anchor="left" open={state["left"]} onClose={toggleDrawer(false)}>
        <ul>
          <li>Uno</li>
          <li>Dos</li>
          <li>Tres</li>
        </ul>
      </Drawer>
    </Container>
  )
}
