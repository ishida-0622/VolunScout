import { Header } from "@/components/layouts/Header";

const Layout = ({ children }: { children: React.ReactNode }) => {
  return (
    <>
      <Header accountType="participant" />
      {children}
    </>
  );
};

export default Layout;
