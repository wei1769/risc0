import { redirect } from "next/navigation";
import type { ReactNode } from "react";
import ProgressProvider from "shared/client/providers/progress-provider";
import { Footer } from "./_components/footer";
import { Header } from "./_components/header";

export default function ReportsLayout({ children, params }: { children: ReactNode; params: { version: string } }) {
  const { version } = params;

  return (
    <>
      <Header version={version} />

      <main className="grow">{children}</main>

      <Footer />

      <ProgressProvider />
    </>
  );
}
