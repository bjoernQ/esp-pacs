#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
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
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_START_INT_ENA` reader - "]
pub type RX_START_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RX_START_INT_ENA` writer - "]
pub type RX_START_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 0>;
#[doc = "Field `TX_START_INT_ENA` reader - "]
pub type TX_START_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TX_START_INT_ENA` writer - "]
pub type TX_START_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 1>;
#[doc = "Field `RX_HUNG_INT_ENA` reader - "]
pub type RX_HUNG_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RX_HUNG_INT_ENA` writer - "]
pub type RX_HUNG_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 2>;
#[doc = "Field `TX_HUNG_INT_ENA` reader - "]
pub type TX_HUNG_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TX_HUNG_INT_ENA` writer - "]
pub type TX_HUNG_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 3>;
#[doc = "Field `IN_DONE_INT_ENA` reader - "]
pub type IN_DONE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `IN_DONE_INT_ENA` writer - "]
pub type IN_DONE_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 4>;
#[doc = "Field `IN_SUC_EOF_INT_ENA` reader - "]
pub type IN_SUC_EOF_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `IN_SUC_EOF_INT_ENA` writer - "]
pub type IN_SUC_EOF_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 5>;
#[doc = "Field `IN_ERR_EOF_INT_ENA` reader - "]
pub type IN_ERR_EOF_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `IN_ERR_EOF_INT_ENA` writer - "]
pub type IN_ERR_EOF_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 6>;
#[doc = "Field `OUT_DONE_INT_ENA` reader - "]
pub type OUT_DONE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OUT_DONE_INT_ENA` writer - "]
pub type OUT_DONE_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 7>;
#[doc = "Field `OUT_EOF_INT_ENA` reader - "]
pub type OUT_EOF_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OUT_EOF_INT_ENA` writer - "]
pub type OUT_EOF_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 8>;
#[doc = "Field `IN_DSCR_ERR_INT_ENA` reader - "]
pub type IN_DSCR_ERR_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `IN_DSCR_ERR_INT_ENA` writer - "]
pub type IN_DSCR_ERR_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 9>;
#[doc = "Field `OUT_DSCR_ERR_INT_ENA` reader - "]
pub type OUT_DSCR_ERR_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OUT_DSCR_ERR_INT_ENA` writer - "]
pub type OUT_DSCR_ERR_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 10>;
#[doc = "Field `IN_DSCR_EMPTY_INT_ENA` reader - "]
pub type IN_DSCR_EMPTY_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `IN_DSCR_EMPTY_INT_ENA` writer - "]
pub type IN_DSCR_EMPTY_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 11>;
#[doc = "Field `OUTLINK_EOF_ERR_INT_ENA` reader - "]
pub type OUTLINK_EOF_ERR_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OUTLINK_EOF_ERR_INT_ENA` writer - "]
pub type OUTLINK_EOF_ERR_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 12>;
#[doc = "Field `OUT_TOTAL_EOF_INT_ENA` reader - "]
pub type OUT_TOTAL_EOF_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OUT_TOTAL_EOF_INT_ENA` writer - "]
pub type OUT_TOTAL_EOF_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 13>;
#[doc = "Field `SEND_S_Q_INT_ENA` reader - "]
pub type SEND_S_Q_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SEND_S_Q_INT_ENA` writer - "]
pub type SEND_S_Q_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 14>;
#[doc = "Field `SEND_A_Q_INT_ENA` reader - "]
pub type SEND_A_Q_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SEND_A_Q_INT_ENA` writer - "]
pub type SEND_A_Q_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 15>;
#[doc = "Field `DMA_INFIFO_FULL_WM_INT_ENA` reader - "]
pub type DMA_INFIFO_FULL_WM_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `DMA_INFIFO_FULL_WM_INT_ENA` writer - "]
pub type DMA_INFIFO_FULL_WM_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_start_int_ena(&self) -> RX_START_INT_ENA_R {
        RX_START_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_start_int_ena(&self) -> TX_START_INT_ENA_R {
        TX_START_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_hung_int_ena(&self) -> RX_HUNG_INT_ENA_R {
        RX_HUNG_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_hung_int_ena(&self) -> TX_HUNG_INT_ENA_R {
        TX_HUNG_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn in_done_int_ena(&self) -> IN_DONE_INT_ENA_R {
        IN_DONE_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn in_suc_eof_int_ena(&self) -> IN_SUC_EOF_INT_ENA_R {
        IN_SUC_EOF_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn in_err_eof_int_ena(&self) -> IN_ERR_EOF_INT_ENA_R {
        IN_ERR_EOF_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn out_done_int_ena(&self) -> OUT_DONE_INT_ENA_R {
        OUT_DONE_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn out_eof_int_ena(&self) -> OUT_EOF_INT_ENA_R {
        OUT_EOF_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn in_dscr_err_int_ena(&self) -> IN_DSCR_ERR_INT_ENA_R {
        IN_DSCR_ERR_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn out_dscr_err_int_ena(&self) -> OUT_DSCR_ERR_INT_ENA_R {
        OUT_DSCR_ERR_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn in_dscr_empty_int_ena(&self) -> IN_DSCR_EMPTY_INT_ENA_R {
        IN_DSCR_EMPTY_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn outlink_eof_err_int_ena(&self) -> OUTLINK_EOF_ERR_INT_ENA_R {
        OUTLINK_EOF_ERR_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn out_total_eof_int_ena(&self) -> OUT_TOTAL_EOF_INT_ENA_R {
        OUT_TOTAL_EOF_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn send_s_q_int_ena(&self) -> SEND_S_Q_INT_ENA_R {
        SEND_S_Q_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn send_a_q_int_ena(&self) -> SEND_A_Q_INT_ENA_R {
        SEND_A_Q_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dma_infifo_full_wm_int_ena(&self) -> DMA_INFIFO_FULL_WM_INT_ENA_R {
        DMA_INFIFO_FULL_WM_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_start_int_ena(&mut self) -> RX_START_INT_ENA_W {
        RX_START_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_start_int_ena(&mut self) -> TX_START_INT_ENA_W {
        TX_START_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_hung_int_ena(&mut self) -> RX_HUNG_INT_ENA_W {
        RX_HUNG_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_hung_int_ena(&mut self) -> TX_HUNG_INT_ENA_W {
        TX_HUNG_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn in_done_int_ena(&mut self) -> IN_DONE_INT_ENA_W {
        IN_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn in_suc_eof_int_ena(&mut self) -> IN_SUC_EOF_INT_ENA_W {
        IN_SUC_EOF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn in_err_eof_int_ena(&mut self) -> IN_ERR_EOF_INT_ENA_W {
        IN_ERR_EOF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn out_done_int_ena(&mut self) -> OUT_DONE_INT_ENA_W {
        OUT_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn out_eof_int_ena(&mut self) -> OUT_EOF_INT_ENA_W {
        OUT_EOF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn in_dscr_err_int_ena(&mut self) -> IN_DSCR_ERR_INT_ENA_W {
        IN_DSCR_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn out_dscr_err_int_ena(&mut self) -> OUT_DSCR_ERR_INT_ENA_W {
        OUT_DSCR_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn in_dscr_empty_int_ena(&mut self) -> IN_DSCR_EMPTY_INT_ENA_W {
        IN_DSCR_EMPTY_INT_ENA_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn outlink_eof_err_int_ena(&mut self) -> OUTLINK_EOF_ERR_INT_ENA_W {
        OUTLINK_EOF_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn out_total_eof_int_ena(&mut self) -> OUT_TOTAL_EOF_INT_ENA_W {
        OUT_TOTAL_EOF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn send_s_q_int_ena(&mut self) -> SEND_S_Q_INT_ENA_W {
        SEND_S_Q_INT_ENA_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn send_a_q_int_ena(&mut self) -> SEND_A_Q_INT_ENA_W {
        SEND_A_Q_INT_ENA_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dma_infifo_full_wm_int_ena(&mut self) -> DMA_INFIFO_FULL_WM_INT_ENA_W {
        DMA_INFIFO_FULL_WM_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
