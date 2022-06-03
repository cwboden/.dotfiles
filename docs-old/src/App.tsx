import * as React from "react"
import { ChakraProvider, extendTheme } from "@chakra-ui/react"
import { NavBar } from "./NavBar"

const colors = {
  brand: {
      700: "#BE961E",
      300: "#503E2C",
  },
}

const theme = extendTheme({ colors })

export const App = () => (
  <ChakraProvider theme={theme}>
    <NavBar />
  </ChakraProvider>
)
