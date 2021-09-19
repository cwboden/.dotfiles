import { Box } from "@chakra-ui/react"
import { MdClose, MdMenu } from "react-icons/md"

type Props = {
  toggle: () => void,
  isOpen: boolean
}

export const MenuToggle = (props: Props) => {
  return (
    <Box display={{ base: "block", md: "none" }} onClick={props.toggle}>
      {props.isOpen ? <MdClose /> : <MdMenu />}
    </Box>
  )
}


