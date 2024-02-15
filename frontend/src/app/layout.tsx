import type { Metadata } from "next";

import { Footer } from "@/components/layouts/Footer";
import { Header } from "@/components/layouts/Header";
import { IconConfig } from "@/components/layouts/IconConfig/index";
import { WithApollo } from "@/components/layouts/WithApollo";
import { AuthProvider } from "@/contexts/AuthContext";

import "bootstrap/dist/css/bootstrap.min.css";
import "./globals.css";

export const metadata: Metadata = {
  title: "VolunScout",
  description: "VolunScout is a platform for volunteers and NPOs.",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="ja">
      <body>
        <WithApollo>
          <AuthProvider>
            <IconConfig>
              <Header />
              {children}
            </IconConfig>
          </AuthProvider>
        </WithApollo>
        <Footer />
      </body>
    </html>
  );
}
