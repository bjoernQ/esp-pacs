#[doc = "Register `FIFO_CONF` reader"]
pub type R = crate::R<FIFO_CONF_SPEC>;
#[doc = "Register `FIFO_CONF` writer"]
pub type W = crate::W<FIFO_CONF_SPEC>;
#[doc = "Field `RXFIFO_WM_THRHD` reader - The water mark threshold of rx FIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and rx FIFO counter is bigger than reg_rxfifo_wm_thrhd\\[3:0\\], reg_rxfifo_wm_int_raw bit will be valid."]
pub type RXFIFO_WM_THRHD_R = crate::FieldReader;
#[doc = "Field `RXFIFO_WM_THRHD` writer - The water mark threshold of rx FIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and rx FIFO counter is bigger than reg_rxfifo_wm_thrhd\\[3:0\\], reg_rxfifo_wm_int_raw bit will be valid."]
pub type RXFIFO_WM_THRHD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TXFIFO_WM_THRHD` reader - The water mark threshold of tx FIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and tx FIFO counter is smaller than reg_txfifo_wm_thrhd\\[3:0\\], reg_txfifo_wm_int_raw bit will be valid."]
pub type TXFIFO_WM_THRHD_R = crate::FieldReader;
#[doc = "Field `TXFIFO_WM_THRHD` writer - The water mark threshold of tx FIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and tx FIFO counter is smaller than reg_txfifo_wm_thrhd\\[3:0\\], reg_txfifo_wm_int_raw bit will be valid."]
pub type TXFIFO_WM_THRHD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `NONFIFO_EN` reader - Set this bit to enable APB nonfifo access."]
pub type NONFIFO_EN_R = crate::BitReader;
#[doc = "Field `NONFIFO_EN` writer - Set this bit to enable APB nonfifo access."]
pub type NONFIFO_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_FIFO_RST` reader - Set this bit to reset rx-fifo."]
pub type RX_FIFO_RST_R = crate::BitReader;
#[doc = "Field `RX_FIFO_RST` writer - Set this bit to reset rx-fifo."]
pub type RX_FIFO_RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_FIFO_RST` reader - Set this bit to reset tx-fifo."]
pub type TX_FIFO_RST_R = crate::BitReader;
#[doc = "Field `TX_FIFO_RST` writer - Set this bit to reset tx-fifo."]
pub type TX_FIFO_RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FIFO_PRT_EN` reader - The control enable bit of FIFO pointer in non-fifo access mode. This bit controls the valid bits and the interrupts of tx/rx_fifo overflow, underflow, full and empty."]
pub type FIFO_PRT_EN_R = crate::BitReader;
#[doc = "Field `FIFO_PRT_EN` writer - The control enable bit of FIFO pointer in non-fifo access mode. This bit controls the valid bits and the interrupts of tx/rx_fifo overflow, underflow, full and empty."]
pub type FIFO_PRT_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - The water mark threshold of rx FIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and rx FIFO counter is bigger than reg_rxfifo_wm_thrhd\\[3:0\\], reg_rxfifo_wm_int_raw bit will be valid."]
    #[inline(always)]
    pub fn rxfifo_wm_thrhd(&self) -> RXFIFO_WM_THRHD_R {
        RXFIFO_WM_THRHD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 5:8 - The water mark threshold of tx FIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and tx FIFO counter is smaller than reg_txfifo_wm_thrhd\\[3:0\\], reg_txfifo_wm_int_raw bit will be valid."]
    #[inline(always)]
    pub fn txfifo_wm_thrhd(&self) -> TXFIFO_WM_THRHD_R {
        TXFIFO_WM_THRHD_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - Set this bit to enable APB nonfifo access."]
    #[inline(always)]
    pub fn nonfifo_en(&self) -> NONFIFO_EN_R {
        NONFIFO_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to reset rx-fifo."]
    #[inline(always)]
    pub fn rx_fifo_rst(&self) -> RX_FIFO_RST_R {
        RX_FIFO_RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set this bit to reset tx-fifo."]
    #[inline(always)]
    pub fn tx_fifo_rst(&self) -> TX_FIFO_RST_R {
        TX_FIFO_RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The control enable bit of FIFO pointer in non-fifo access mode. This bit controls the valid bits and the interrupts of tx/rx_fifo overflow, underflow, full and empty."]
    #[inline(always)]
    pub fn fifo_prt_en(&self) -> FIFO_PRT_EN_R {
        FIFO_PRT_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_CONF")
            .field(
                "rxfifo_wm_thrhd",
                &format_args!("{}", self.rxfifo_wm_thrhd().bits()),
            )
            .field(
                "txfifo_wm_thrhd",
                &format_args!("{}", self.txfifo_wm_thrhd().bits()),
            )
            .field("nonfifo_en", &format_args!("{}", self.nonfifo_en().bit()))
            .field("rx_fifo_rst", &format_args!("{}", self.rx_fifo_rst().bit()))
            .field("tx_fifo_rst", &format_args!("{}", self.tx_fifo_rst().bit()))
            .field("fifo_prt_en", &format_args!("{}", self.fifo_prt_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FIFO_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - The water mark threshold of rx FIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and rx FIFO counter is bigger than reg_rxfifo_wm_thrhd\\[3:0\\], reg_rxfifo_wm_int_raw bit will be valid."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_wm_thrhd(&mut self) -> RXFIFO_WM_THRHD_W<FIFO_CONF_SPEC, 0> {
        RXFIFO_WM_THRHD_W::new(self)
    }
    #[doc = "Bits 5:8 - The water mark threshold of tx FIFO in nonfifo access mode. When reg_reg_fifo_prt_en is 1 and tx FIFO counter is smaller than reg_txfifo_wm_thrhd\\[3:0\\], reg_txfifo_wm_int_raw bit will be valid."]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_wm_thrhd(&mut self) -> TXFIFO_WM_THRHD_W<FIFO_CONF_SPEC, 5> {
        TXFIFO_WM_THRHD_W::new(self)
    }
    #[doc = "Bit 10 - Set this bit to enable APB nonfifo access."]
    #[inline(always)]
    #[must_use]
    pub fn nonfifo_en(&mut self) -> NONFIFO_EN_W<FIFO_CONF_SPEC, 10> {
        NONFIFO_EN_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to reset rx-fifo."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_rst(&mut self) -> RX_FIFO_RST_W<FIFO_CONF_SPEC, 12> {
        RX_FIFO_RST_W::new(self)
    }
    #[doc = "Bit 13 - Set this bit to reset tx-fifo."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_rst(&mut self) -> TX_FIFO_RST_W<FIFO_CONF_SPEC, 13> {
        TX_FIFO_RST_W::new(self)
    }
    #[doc = "Bit 14 - The control enable bit of FIFO pointer in non-fifo access mode. This bit controls the valid bits and the interrupts of tx/rx_fifo overflow, underflow, full and empty."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_prt_en(&mut self) -> FIFO_PRT_EN_W<FIFO_CONF_SPEC, 14> {
        FIFO_PRT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FIFO configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_CONF_SPEC;
impl crate::RegisterSpec for FIFO_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_conf::R`](R) reader structure"]
impl crate::Readable for FIFO_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo_conf::W`](W) writer structure"]
impl crate::Writable for FIFO_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFO_CONF to value 0x4046"]
impl crate::Resettable for FIFO_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x4046;
}
