#[doc = "Register `INT_CLR` writer"]
pub struct W(crate::W<INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_SPEC>;
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
impl From<crate::W<INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_START_INT_CLR` writer - "]
pub type RX_START_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 0>;
#[doc = "Field `TX_START_INT_CLR` writer - "]
pub type TX_START_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 1>;
#[doc = "Field `RX_HUNG_INT_CLR` writer - "]
pub type RX_HUNG_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 2>;
#[doc = "Field `TX_HUNG_INT_CLR` writer - "]
pub type TX_HUNG_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 3>;
#[doc = "Field `IN_DONE_INT_CLR` writer - "]
pub type IN_DONE_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 4>;
#[doc = "Field `IN_SUC_EOF_INT_CLR` writer - "]
pub type IN_SUC_EOF_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 5>;
#[doc = "Field `IN_ERR_EOF_INT_CLR` writer - "]
pub type IN_ERR_EOF_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 6>;
#[doc = "Field `OUT_DONE_INT_CLR` writer - "]
pub type OUT_DONE_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 7>;
#[doc = "Field `OUT_EOF_INT_CLR` writer - "]
pub type OUT_EOF_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 8>;
#[doc = "Field `IN_DSCR_ERR_INT_CLR` writer - "]
pub type IN_DSCR_ERR_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 9>;
#[doc = "Field `OUT_DSCR_ERR_INT_CLR` writer - "]
pub type OUT_DSCR_ERR_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 10>;
#[doc = "Field `IN_DSCR_EMPTY_INT_CLR` writer - "]
pub type IN_DSCR_EMPTY_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 11>;
#[doc = "Field `OUTLINK_EOF_ERR_INT_CLR` writer - "]
pub type OUTLINK_EOF_ERR_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 12>;
#[doc = "Field `OUT_TOTAL_EOF_INT_CLR` writer - "]
pub type OUT_TOTAL_EOF_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 13>;
#[doc = "Field `SEND_S_Q_INT_CLR` writer - "]
pub type SEND_S_Q_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 14>;
#[doc = "Field `SEND_A_Q_INT_CLR` writer - "]
pub type SEND_A_Q_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 15>;
#[doc = "Field `DMA_INFIFO_FULL_WM_INT_CLR` writer - "]
pub type DMA_INFIFO_FULL_WM_INT_CLR_W<'a> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, 16>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_start_int_clr(&mut self) -> RX_START_INT_CLR_W {
        RX_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_start_int_clr(&mut self) -> TX_START_INT_CLR_W {
        TX_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_hung_int_clr(&mut self) -> RX_HUNG_INT_CLR_W {
        RX_HUNG_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_hung_int_clr(&mut self) -> TX_HUNG_INT_CLR_W {
        TX_HUNG_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn in_done_int_clr(&mut self) -> IN_DONE_INT_CLR_W {
        IN_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn in_suc_eof_int_clr(&mut self) -> IN_SUC_EOF_INT_CLR_W {
        IN_SUC_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn in_err_eof_int_clr(&mut self) -> IN_ERR_EOF_INT_CLR_W {
        IN_ERR_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn out_done_int_clr(&mut self) -> OUT_DONE_INT_CLR_W {
        OUT_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn out_eof_int_clr(&mut self) -> OUT_EOF_INT_CLR_W {
        OUT_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn in_dscr_err_int_clr(&mut self) -> IN_DSCR_ERR_INT_CLR_W {
        IN_DSCR_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn out_dscr_err_int_clr(&mut self) -> OUT_DSCR_ERR_INT_CLR_W {
        OUT_DSCR_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn in_dscr_empty_int_clr(&mut self) -> IN_DSCR_EMPTY_INT_CLR_W {
        IN_DSCR_EMPTY_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn outlink_eof_err_int_clr(&mut self) -> OUTLINK_EOF_ERR_INT_CLR_W {
        OUTLINK_EOF_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn out_total_eof_int_clr(&mut self) -> OUT_TOTAL_EOF_INT_CLR_W {
        OUT_TOTAL_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn send_s_q_int_clr(&mut self) -> SEND_S_Q_INT_CLR_W {
        SEND_S_Q_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn send_a_q_int_clr(&mut self) -> SEND_A_Q_INT_CLR_W {
        SEND_A_Q_INT_CLR_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dma_infifo_full_wm_int_clr(&mut self) -> DMA_INFIFO_FULL_WM_INT_CLR_W {
        DMA_INFIFO_FULL_WM_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
