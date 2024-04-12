import { Link } from "@risc0/ui/link";
import compact from "lodash-es/compact";
import { headers } from "next/headers";

export function LinkWithVersion({ href, ...rest }) {
  const pathname = headers().get("x-pathname")!;
  const version = compact(pathname.split("/"))[0];

  return <Link href={`/${version}${href}`} {...rest} />;
}
