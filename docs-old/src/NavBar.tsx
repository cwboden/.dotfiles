import { useState } from "react"
import { Flex } from "@chakra-ui/react"
import { Logo } from "./Logo"
import { MenuToggle } from "./MenuToggle"
import { MenuLinks } from "./MenuLinks"

export const NavBar = () => {
  const [isOpen, setIsOpen] = useState(false)
  const toggle = () => setIsOpen(!isOpen)

  return(
    <NavBarContainer>
      <Logo w="100px"/>
      <MenuToggle toggle={toggle} isOpen={isOpen} />
      <MenuLinks isOpen={isOpen} />
    </NavBarContainer>
  )
}

const NavBarContainer = ({children}) => {
  return (
    <Flex
      as="nav"
      align="center"
      justify="space-between"
      wrap="wrap"
      w="100%"
      mb={8}
      p={8}
    >
      {children}
    </Flex>
  )
}
