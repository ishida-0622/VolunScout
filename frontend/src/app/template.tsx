import { Header } from "@/components/layouts/Header";

const Template = ({ children }: { children: React.ReactNode }) => {
  return (
    <>
      <Header />
      {children}
    </>
  );
};

export default Template;
