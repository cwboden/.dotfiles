import { Link, Text } from "@chakra-ui/react"

export const MenuItem = ({linkTo, children}) => {
  return (
    <Link href={linkTo}>
      <Text display="block">
        {children}
      </Text>
    </Link>
  )
}
