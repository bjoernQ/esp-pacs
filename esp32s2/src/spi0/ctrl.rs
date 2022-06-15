#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXT_HOLD_EN` reader - Set the bit to hold spi. The bit is combined with SPI_USR_PREP_HOLD,SPI_USR_CMD_HOLD,SPI_USR_ADDR_HOLD,SPI_USR_DUMMY_HOLD,SPI_USR_DIN_HOLD,SPI_USR_DOUT_HOLD and SPI_USR_HOLD_POL. Can be configured in CONF state."]
pub type EXT_HOLD_EN_R = crate::BitReader<bool>;
#[doc = "Field `EXT_HOLD_EN` writer - Set the bit to hold spi. The bit is combined with SPI_USR_PREP_HOLD,SPI_USR_CMD_HOLD,SPI_USR_ADDR_HOLD,SPI_USR_DUMMY_HOLD,SPI_USR_DIN_HOLD,SPI_USR_DOUT_HOLD and SPI_USR_HOLD_POL. Can be configured in CONF state."]
pub type EXT_HOLD_EN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 2>;
#[doc = "Field `DUMMY_OUT` reader - In the dummy phase the signal level of spi is output by the spi controller. Can be configured in CONF state."]
pub type DUMMY_OUT_R = crate::BitReader<bool>;
#[doc = "Field `DUMMY_OUT` writer - In the dummy phase the signal level of spi is output by the spi controller. Can be configured in CONF state."]
pub type DUMMY_OUT_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 3>;
#[doc = "Field `FADDR_DUAL` reader - Apply 2-bit mode during addr phase 1:enable 0: disable. Can be configured in CONF state."]
pub type FADDR_DUAL_R = crate::BitReader<bool>;
#[doc = "Field `FADDR_DUAL` writer - Apply 2-bit mode during addr phase 1:enable 0: disable. Can be configured in CONF state."]
pub type FADDR_DUAL_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 5>;
#[doc = "Field `FADDR_QUAD` reader - Apply 4-bit mode during addr phase 1:enable 0: disable. Can be configured in CONF state."]
pub type FADDR_QUAD_R = crate::BitReader<bool>;
#[doc = "Field `FADDR_QUAD` writer - Apply 4-bit mode during addr phase 1:enable 0: disable. Can be configured in CONF state."]
pub type FADDR_QUAD_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 6>;
#[doc = "Field `FADDR_OCT` reader - Apply 8-bit mode during addr phase 1:enable 0: disable. Can be configured in CONF state."]
pub type FADDR_OCT_R = crate::BitReader<bool>;
#[doc = "Field `FADDR_OCT` writer - Apply 8-bit mode during addr phase 1:enable 0: disable. Can be configured in CONF state."]
pub type FADDR_OCT_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 7>;
#[doc = "Field `FCMD_DUAL` reader - Apply 2-bit mode during command phase 1:enable 0: disable. Can be configured in CONF state."]
pub type FCMD_DUAL_R = crate::BitReader<bool>;
#[doc = "Field `FCMD_DUAL` writer - Apply 2-bit mode during command phase 1:enable 0: disable. Can be configured in CONF state."]
pub type FCMD_DUAL_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 8>;
#[doc = "Field `FCMD_QUAD` reader - Apply 4-bit mode during command phase 1:enable 0: disable. Can be configured in CONF state."]
pub type FCMD_QUAD_R = crate::BitReader<bool>;
#[doc = "Field `FCMD_QUAD` writer - Apply 4-bit mode during command phase 1:enable 0: disable. Can be configured in CONF state."]
pub type FCMD_QUAD_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 9>;
#[doc = "Field `FCMD_OCT` reader - Apply 8-bit mode during command phase 1:enable 0: disable. Can be configured in CONF state."]
pub type FCMD_OCT_R = crate::BitReader<bool>;
#[doc = "Field `FCMD_OCT` writer - Apply 8-bit mode during command phase 1:enable 0: disable. Can be configured in CONF state."]
pub type FCMD_OCT_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 10>;
#[doc = "Field `FREAD_DUAL` reader - In the read operations, read-data phase is in 2-bit mode. 1: enable 0: disable. Can be configured in CONF state."]
pub type FREAD_DUAL_R = crate::BitReader<bool>;
#[doc = "Field `FREAD_DUAL` writer - In the read operations, read-data phase is in 2-bit mode. 1: enable 0: disable. Can be configured in CONF state."]
pub type FREAD_DUAL_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 14>;
#[doc = "Field `FREAD_QUAD` reader - In the read operations read-data phase is in 4-bit mode. 1: enable 0: disable. Can be configured in CONF state."]
pub type FREAD_QUAD_R = crate::BitReader<bool>;
#[doc = "Field `FREAD_QUAD` writer - In the read operations read-data phase is in 4-bit mode. 1: enable 0: disable. Can be configured in CONF state."]
pub type FREAD_QUAD_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 15>;
#[doc = "Field `FREAD_OCT` reader - In the read operations read-data phase is in 8-bit mode. 1: enable 0: disable. Can be configured in CONF state."]
pub type FREAD_OCT_R = crate::BitReader<bool>;
#[doc = "Field `FREAD_OCT` writer - In the read operations read-data phase is in 8-bit mode. 1: enable 0: disable. Can be configured in CONF state."]
pub type FREAD_OCT_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 16>;
#[doc = "Field `Q_POL` reader - The bit is used to set MISO line polarity, 1: high 0, low. Can be configured in CONF state."]
pub type Q_POL_R = crate::BitReader<bool>;
#[doc = "Field `Q_POL` writer - The bit is used to set MISO line polarity, 1: high 0, low. Can be configured in CONF state."]
pub type Q_POL_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 18>;
#[doc = "Field `D_POL` reader - The bit is used to set MOSI line polarity, 1: high 0, low. Can be configured in CONF state."]
pub type D_POL_R = crate::BitReader<bool>;
#[doc = "Field `D_POL` writer - The bit is used to set MOSI line polarity, 1: high 0, low. Can be configured in CONF state."]
pub type D_POL_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 19>;
#[doc = "Field `WP` reader - Write protect signal output when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state."]
pub type WP_R = crate::BitReader<bool>;
#[doc = "Field `WP` writer - Write protect signal output when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state."]
pub type WP_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 21>;
#[doc = "Field `RD_BIT_ORDER` reader - In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state."]
pub type RD_BIT_ORDER_R = crate::BitReader<bool>;
#[doc = "Field `RD_BIT_ORDER` writer - In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state."]
pub type RD_BIT_ORDER_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 25>;
#[doc = "Field `WR_BIT_ORDER` reader - In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state."]
pub type WR_BIT_ORDER_R = crate::BitReader<bool>;
#[doc = "Field `WR_BIT_ORDER` writer - In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state."]
pub type WR_BIT_ORDER_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 26>;
impl R {
    #[doc = "Bit 2 - Set the bit to hold spi. The bit is combined with SPI_USR_PREP_HOLD,SPI_USR_CMD_HOLD,SPI_USR_ADDR_HOLD,SPI_USR_DUMMY_HOLD,SPI_USR_DIN_HOLD,SPI_USR_DOUT_HOLD and SPI_USR_HOLD_POL. Can be configured in CONF state."]
    #[inline(always)]
    pub fn ext_hold_en(&self) -> EXT_HOLD_EN_R {
        EXT_HOLD_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - In the dummy phase the signal level of spi is output by the spi controller. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dummy_out(&self) -> DUMMY_OUT_R {
        DUMMY_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Apply 2-bit mode during addr phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn faddr_dual(&self) -> FADDR_DUAL_R {
        FADDR_DUAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Apply 4-bit mode during addr phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn faddr_quad(&self) -> FADDR_QUAD_R {
        FADDR_QUAD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Apply 8-bit mode during addr phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn faddr_oct(&self) -> FADDR_OCT_R {
        FADDR_OCT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Apply 2-bit mode during command phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fcmd_dual(&self) -> FCMD_DUAL_R {
        FCMD_DUAL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Apply 4-bit mode during command phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fcmd_quad(&self) -> FCMD_QUAD_R {
        FCMD_QUAD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Apply 8-bit mode during command phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fcmd_oct(&self) -> FCMD_OCT_R {
        FCMD_OCT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - In the read operations, read-data phase is in 2-bit mode. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fread_dual(&self) -> FREAD_DUAL_R {
        FREAD_DUAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - In the read operations read-data phase is in 4-bit mode. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fread_quad(&self) -> FREAD_QUAD_R {
        FREAD_QUAD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - In the read operations read-data phase is in 8-bit mode. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fread_oct(&self) -> FREAD_OCT_R {
        FREAD_OCT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - The bit is used to set MISO line polarity, 1: high 0, low. Can be configured in CONF state."]
    #[inline(always)]
    pub fn q_pol(&self) -> Q_POL_R {
        Q_POL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The bit is used to set MOSI line polarity, 1: high 0, low. Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_pol(&self) -> D_POL_R {
        D_POL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Write protect signal output when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state."]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 25 - In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state."]
    #[inline(always)]
    pub fn rd_bit_order(&self) -> RD_BIT_ORDER_R {
        RD_BIT_ORDER_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state."]
    #[inline(always)]
    pub fn wr_bit_order(&self) -> WR_BIT_ORDER_R {
        WR_BIT_ORDER_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Set the bit to hold spi. The bit is combined with SPI_USR_PREP_HOLD,SPI_USR_CMD_HOLD,SPI_USR_ADDR_HOLD,SPI_USR_DUMMY_HOLD,SPI_USR_DIN_HOLD,SPI_USR_DOUT_HOLD and SPI_USR_HOLD_POL. Can be configured in CONF state."]
    #[inline(always)]
    pub fn ext_hold_en(&mut self) -> EXT_HOLD_EN_W {
        EXT_HOLD_EN_W::new(self)
    }
    #[doc = "Bit 3 - In the dummy phase the signal level of spi is output by the spi controller. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dummy_out(&mut self) -> DUMMY_OUT_W {
        DUMMY_OUT_W::new(self)
    }
    #[doc = "Bit 5 - Apply 2-bit mode during addr phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn faddr_dual(&mut self) -> FADDR_DUAL_W {
        FADDR_DUAL_W::new(self)
    }
    #[doc = "Bit 6 - Apply 4-bit mode during addr phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn faddr_quad(&mut self) -> FADDR_QUAD_W {
        FADDR_QUAD_W::new(self)
    }
    #[doc = "Bit 7 - Apply 8-bit mode during addr phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn faddr_oct(&mut self) -> FADDR_OCT_W {
        FADDR_OCT_W::new(self)
    }
    #[doc = "Bit 8 - Apply 2-bit mode during command phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fcmd_dual(&mut self) -> FCMD_DUAL_W {
        FCMD_DUAL_W::new(self)
    }
    #[doc = "Bit 9 - Apply 4-bit mode during command phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fcmd_quad(&mut self) -> FCMD_QUAD_W {
        FCMD_QUAD_W::new(self)
    }
    #[doc = "Bit 10 - Apply 8-bit mode during command phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fcmd_oct(&mut self) -> FCMD_OCT_W {
        FCMD_OCT_W::new(self)
    }
    #[doc = "Bit 14 - In the read operations, read-data phase is in 2-bit mode. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fread_dual(&mut self) -> FREAD_DUAL_W {
        FREAD_DUAL_W::new(self)
    }
    #[doc = "Bit 15 - In the read operations read-data phase is in 4-bit mode. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fread_quad(&mut self) -> FREAD_QUAD_W {
        FREAD_QUAD_W::new(self)
    }
    #[doc = "Bit 16 - In the read operations read-data phase is in 8-bit mode. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fread_oct(&mut self) -> FREAD_OCT_W {
        FREAD_OCT_W::new(self)
    }
    #[doc = "Bit 18 - The bit is used to set MISO line polarity, 1: high 0, low. Can be configured in CONF state."]
    #[inline(always)]
    pub fn q_pol(&mut self) -> Q_POL_W {
        Q_POL_W::new(self)
    }
    #[doc = "Bit 19 - The bit is used to set MOSI line polarity, 1: high 0, low. Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_pol(&mut self) -> D_POL_W {
        D_POL_W::new(self)
    }
    #[doc = "Bit 21 - Write protect signal output when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state."]
    #[inline(always)]
    pub fn wp(&mut self) -> WP_W {
        WP_W::new(self)
    }
    #[doc = "Bit 25 - In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state."]
    #[inline(always)]
    pub fn rd_bit_order(&mut self) -> RD_BIT_ORDER_W {
        RD_BIT_ORDER_W::new(self)
    }
    #[doc = "Bit 26 - In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state."]
    #[inline(always)]
    pub fn wr_bit_order(&mut self) -> WR_BIT_ORDER_W {
        WR_BIT_ORDER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x002c_0000"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x002c_0000
    }
}
