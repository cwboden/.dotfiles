import { Box, Stack } from "@chakra-ui/react"
import { MenuItem } from "./MenuItem"

export const MenuLinks = ({isOpen}) => {
  return (
    <Box
      display={{ base: isOpen ? "block" : "none", md: "block" }}
      flexBasis={{ base: "100%", md: "auto" }}
    >
      <Stack
        spacing={8}
        align="center"
        justify={["center", "space-between", "flex-end", "flex-end"]}
        direction={["column", "row", "row", "row"]}
        pt={[4, 4, 0, 0]}
      >
        <MenuItem linkTo="/">Home</MenuItem>
        <MenuItem linkTo="/about-me">About Me</MenuItem>
        <MenuItem linkTo="/projects">Projects</MenuItem>
      </Stack>
    </Box>
  )
}
