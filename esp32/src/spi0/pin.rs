#[doc = "Register `PIN` reader"]
pub struct R(crate::R<PIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIN` writer"]
pub struct W(crate::W<PIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS0_DIS` reader - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin"]
pub type CS0_DIS_R = crate::BitReader<bool>;
#[doc = "Field `CS0_DIS` writer - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin"]
pub type CS0_DIS_W<'a> = crate::BitWriter<'a, u32, PIN_SPEC, bool, 0>;
#[doc = "Field `CS1_DIS` reader - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin"]
pub type CS1_DIS_R = crate::BitReader<bool>;
#[doc = "Field `CS1_DIS` writer - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin"]
pub type CS1_DIS_W<'a> = crate::BitWriter<'a, u32, PIN_SPEC, bool, 1>;
#[doc = "Field `CS2_DIS` reader - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin"]
pub type CS2_DIS_R = crate::BitReader<bool>;
#[doc = "Field `CS2_DIS` writer - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin"]
pub type CS2_DIS_W<'a> = crate::BitWriter<'a, u32, PIN_SPEC, bool, 2>;
#[doc = "Field `CK_DIS` reader - 1: spi clk out disable 0: spi clk out enable"]
pub type CK_DIS_R = crate::BitReader<bool>;
#[doc = "Field `CK_DIS` writer - 1: spi clk out disable 0: spi clk out enable"]
pub type CK_DIS_W<'a> = crate::BitWriter<'a, u32, PIN_SPEC, bool, 5>;
#[doc = "Field `MASTER_CS_POL` reader - In the master mode the bits are the polarity of spi cs line the value is equivalent to spi_cs ^ spi_master_cs_pol."]
pub type MASTER_CS_POL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASTER_CS_POL` writer - In the master mode the bits are the polarity of spi cs line the value is equivalent to spi_cs ^ spi_master_cs_pol."]
pub type MASTER_CS_POL_W<'a> = crate::FieldWriter<'a, u32, PIN_SPEC, u8, u8, 3, 6>;
#[doc = "Field `MASTER_CK_SEL` reader - In the master mode spi cs line is enable as spi clk it is combined with spi_cs0_dis spi_cs1_dis spi_cs2_dis."]
pub type MASTER_CK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASTER_CK_SEL` writer - In the master mode spi cs line is enable as spi clk it is combined with spi_cs0_dis spi_cs1_dis spi_cs2_dis."]
pub type MASTER_CK_SEL_W<'a> = crate::FieldWriter<'a, u32, PIN_SPEC, u8, u8, 3, 11>;
#[doc = "Field `CK_IDLE_EDGE` reader - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
pub type CK_IDLE_EDGE_R = crate::BitReader<bool>;
#[doc = "Field `CK_IDLE_EDGE` writer - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
pub type CK_IDLE_EDGE_W<'a> = crate::BitWriter<'a, u32, PIN_SPEC, bool, 29>;
#[doc = "Field `CS_KEEP_ACTIVE` reader - spi cs line keep low when the bit is set."]
pub type CS_KEEP_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `CS_KEEP_ACTIVE` writer - spi cs line keep low when the bit is set."]
pub type CS_KEEP_ACTIVE_W<'a> = crate::BitWriter<'a, u32, PIN_SPEC, bool, 30>;
impl R {
    #[doc = "Bit 0 - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin"]
    #[inline(always)]
    pub fn cs0_dis(&self) -> CS0_DIS_R {
        CS0_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin"]
    #[inline(always)]
    pub fn cs1_dis(&self) -> CS1_DIS_R {
        CS1_DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin"]
    #[inline(always)]
    pub fn cs2_dis(&self) -> CS2_DIS_R {
        CS2_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: spi clk out disable 0: spi clk out enable"]
    #[inline(always)]
    pub fn ck_dis(&self) -> CK_DIS_R {
        CK_DIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - In the master mode the bits are the polarity of spi cs line the value is equivalent to spi_cs ^ spi_master_cs_pol."]
    #[inline(always)]
    pub fn master_cs_pol(&self) -> MASTER_CS_POL_R {
        MASTER_CS_POL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 11:13 - In the master mode spi cs line is enable as spi clk it is combined with spi_cs0_dis spi_cs1_dis spi_cs2_dis."]
    #[inline(always)]
    pub fn master_ck_sel(&self) -> MASTER_CK_SEL_R {
        MASTER_CK_SEL_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 29 - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn ck_idle_edge(&self) -> CK_IDLE_EDGE_R {
        CK_IDLE_EDGE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - spi cs line keep low when the bit is set."]
    #[inline(always)]
    pub fn cs_keep_active(&self) -> CS_KEEP_ACTIVE_R {
        CS_KEEP_ACTIVE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin"]
    #[inline(always)]
    pub fn cs0_dis(&mut self) -> CS0_DIS_W {
        CS0_DIS_W::new(self)
    }
    #[doc = "Bit 1 - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin"]
    #[inline(always)]
    pub fn cs1_dis(&mut self) -> CS1_DIS_W {
        CS1_DIS_W::new(self)
    }
    #[doc = "Bit 2 - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin"]
    #[inline(always)]
    pub fn cs2_dis(&mut self) -> CS2_DIS_W {
        CS2_DIS_W::new(self)
    }
    #[doc = "Bit 5 - 1: spi clk out disable 0: spi clk out enable"]
    #[inline(always)]
    pub fn ck_dis(&mut self) -> CK_DIS_W {
        CK_DIS_W::new(self)
    }
    #[doc = "Bits 6:8 - In the master mode the bits are the polarity of spi cs line the value is equivalent to spi_cs ^ spi_master_cs_pol."]
    #[inline(always)]
    pub fn master_cs_pol(&mut self) -> MASTER_CS_POL_W {
        MASTER_CS_POL_W::new(self)
    }
    #[doc = "Bits 11:13 - In the master mode spi cs line is enable as spi clk it is combined with spi_cs0_dis spi_cs1_dis spi_cs2_dis."]
    #[inline(always)]
    pub fn master_ck_sel(&mut self) -> MASTER_CK_SEL_W {
        MASTER_CK_SEL_W::new(self)
    }
    #[doc = "Bit 29 - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn ck_idle_edge(&mut self) -> CK_IDLE_EDGE_W {
        CK_IDLE_EDGE_W::new(self)
    }
    #[doc = "Bit 30 - spi cs line keep low when the bit is set."]
    #[inline(always)]
    pub fn cs_keep_active(&mut self) -> CS_KEEP_ACTIVE_W {
        CS_KEEP_ACTIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin](index.html) module"]
pub struct PIN_SPEC;
impl crate::RegisterSpec for PIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pin::R](R) reader structure"]
impl crate::Readable for PIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pin::W](W) writer structure"]
impl crate::Writable for PIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIN to value 0x06"]
impl crate::Resettable for PIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06
    }
}
