import { Header } from "@/components/layouts/Header";

const Layout = ({ children }: { children: React.ReactNode }) => {
  return (
    <>
      <Header accountType="group" />
      {children}
    </>
  );
};

export default Layout;
