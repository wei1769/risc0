import { Link } from "@risc0/ui/link";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@risc0/ui/select";

type VersionSelectProps = {
  version: string;
};

export function VersionSelect({ version }: VersionSelectProps) {
  return (
    <Select>
      <SelectTrigger size="sm" className="capitalize">
        <SelectValue placeholder={version} />
      </SelectTrigger>
      <SelectContent>
        <SelectGroup>
          <SelectLabel>Versions</SelectLabel>
          <SelectItem value="latest">
            <Link href="/latest">Latest</Link>
          </SelectItem>
        </SelectGroup>
      </SelectContent>
    </Select>
  );
}
