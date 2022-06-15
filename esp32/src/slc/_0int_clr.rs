#[doc = "Register `_0INT_CLR` writer"]
pub struct W(crate::W<_0INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_0INT_CLR_SPEC>;
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
impl From<crate::W<_0INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_0INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRHOST_BIT0_INT_CLR` writer - "]
pub type FRHOST_BIT0_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 0>;
#[doc = "Field `FRHOST_BIT1_INT_CLR` writer - "]
pub type FRHOST_BIT1_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 1>;
#[doc = "Field `FRHOST_BIT2_INT_CLR` writer - "]
pub type FRHOST_BIT2_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 2>;
#[doc = "Field `FRHOST_BIT3_INT_CLR` writer - "]
pub type FRHOST_BIT3_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 3>;
#[doc = "Field `FRHOST_BIT4_INT_CLR` writer - "]
pub type FRHOST_BIT4_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 4>;
#[doc = "Field `FRHOST_BIT5_INT_CLR` writer - "]
pub type FRHOST_BIT5_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 5>;
#[doc = "Field `FRHOST_BIT6_INT_CLR` writer - "]
pub type FRHOST_BIT6_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 6>;
#[doc = "Field `FRHOST_BIT7_INT_CLR` writer - "]
pub type FRHOST_BIT7_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 7>;
#[doc = "Field `SLC0_RX_START_INT_CLR` writer - "]
pub type SLC0_RX_START_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 8>;
#[doc = "Field `SLC0_TX_START_INT_CLR` writer - "]
pub type SLC0_TX_START_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 9>;
#[doc = "Field `SLC0_RX_UDF_INT_CLR` writer - "]
pub type SLC0_RX_UDF_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 10>;
#[doc = "Field `SLC0_TX_OVF_INT_CLR` writer - "]
pub type SLC0_TX_OVF_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 11>;
#[doc = "Field `SLC0_TOKEN0_1TO0_INT_CLR` writer - "]
pub type SLC0_TOKEN0_1TO0_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 12>;
#[doc = "Field `SLC0_TOKEN1_1TO0_INT_CLR` writer - "]
pub type SLC0_TOKEN1_1TO0_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 13>;
#[doc = "Field `SLC0_TX_DONE_INT_CLR` writer - "]
pub type SLC0_TX_DONE_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 14>;
#[doc = "Field `SLC0_TX_SUC_EOF_INT_CLR` writer - "]
pub type SLC0_TX_SUC_EOF_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 15>;
#[doc = "Field `SLC0_RX_DONE_INT_CLR` writer - "]
pub type SLC0_RX_DONE_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 16>;
#[doc = "Field `SLC0_RX_EOF_INT_CLR` writer - "]
pub type SLC0_RX_EOF_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 17>;
#[doc = "Field `SLC0_TOHOST_INT_CLR` writer - "]
pub type SLC0_TOHOST_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 18>;
#[doc = "Field `SLC0_TX_DSCR_ERR_INT_CLR` writer - "]
pub type SLC0_TX_DSCR_ERR_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 19>;
#[doc = "Field `SLC0_RX_DSCR_ERR_INT_CLR` writer - "]
pub type SLC0_RX_DSCR_ERR_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 20>;
#[doc = "Field `SLC0_TX_DSCR_EMPTY_INT_CLR` writer - "]
pub type SLC0_TX_DSCR_EMPTY_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 21>;
#[doc = "Field `SLC0_HOST_RD_ACK_INT_CLR` writer - "]
pub type SLC0_HOST_RD_ACK_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 22>;
#[doc = "Field `SLC0_WR_RETRY_DONE_INT_CLR` writer - "]
pub type SLC0_WR_RETRY_DONE_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 23>;
#[doc = "Field `SLC0_TX_ERR_EOF_INT_CLR` writer - "]
pub type SLC0_TX_ERR_EOF_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 24>;
#[doc = "Field `CMD_DTC_INT_CLR` writer - "]
pub type CMD_DTC_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 25>;
#[doc = "Field `SLC0_RX_QUICK_EOF_INT_CLR` writer - "]
pub type SLC0_RX_QUICK_EOF_INT_CLR_W<'a> = crate::BitWriter<'a, u32, _0INT_CLR_SPEC, bool, 26>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn frhost_bit0_int_clr(&mut self) -> FRHOST_BIT0_INT_CLR_W {
        FRHOST_BIT0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn frhost_bit1_int_clr(&mut self) -> FRHOST_BIT1_INT_CLR_W {
        FRHOST_BIT1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn frhost_bit2_int_clr(&mut self) -> FRHOST_BIT2_INT_CLR_W {
        FRHOST_BIT2_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn frhost_bit3_int_clr(&mut self) -> FRHOST_BIT3_INT_CLR_W {
        FRHOST_BIT3_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn frhost_bit4_int_clr(&mut self) -> FRHOST_BIT4_INT_CLR_W {
        FRHOST_BIT4_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn frhost_bit5_int_clr(&mut self) -> FRHOST_BIT5_INT_CLR_W {
        FRHOST_BIT5_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn frhost_bit6_int_clr(&mut self) -> FRHOST_BIT6_INT_CLR_W {
        FRHOST_BIT6_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn frhost_bit7_int_clr(&mut self) -> FRHOST_BIT7_INT_CLR_W {
        FRHOST_BIT7_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc0_rx_start_int_clr(&mut self) -> SLC0_RX_START_INT_CLR_W {
        SLC0_RX_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slc0_tx_start_int_clr(&mut self) -> SLC0_TX_START_INT_CLR_W {
        SLC0_TX_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slc0_rx_udf_int_clr(&mut self) -> SLC0_RX_UDF_INT_CLR_W {
        SLC0_RX_UDF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slc0_tx_ovf_int_clr(&mut self) -> SLC0_TX_OVF_INT_CLR_W {
        SLC0_TX_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc0_token0_1to0_int_clr(&mut self) -> SLC0_TOKEN0_1TO0_INT_CLR_W {
        SLC0_TOKEN0_1TO0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc0_token1_1to0_int_clr(&mut self) -> SLC0_TOKEN1_1TO0_INT_CLR_W {
        SLC0_TOKEN1_1TO0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc0_tx_done_int_clr(&mut self) -> SLC0_TX_DONE_INT_CLR_W {
        SLC0_TX_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slc0_tx_suc_eof_int_clr(&mut self) -> SLC0_TX_SUC_EOF_INT_CLR_W {
        SLC0_TX_SUC_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc0_rx_done_int_clr(&mut self) -> SLC0_RX_DONE_INT_CLR_W {
        SLC0_RX_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc0_rx_eof_int_clr(&mut self) -> SLC0_RX_EOF_INT_CLR_W {
        SLC0_RX_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc0_tohost_int_clr(&mut self) -> SLC0_TOHOST_INT_CLR_W {
        SLC0_TOHOST_INT_CLR_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slc0_tx_dscr_err_int_clr(&mut self) -> SLC0_TX_DSCR_ERR_INT_CLR_W {
        SLC0_TX_DSCR_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc0_rx_dscr_err_int_clr(&mut self) -> SLC0_RX_DSCR_ERR_INT_CLR_W {
        SLC0_RX_DSCR_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc0_tx_dscr_empty_int_clr(&mut self) -> SLC0_TX_DSCR_EMPTY_INT_CLR_W {
        SLC0_TX_DSCR_EMPTY_INT_CLR_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn slc0_host_rd_ack_int_clr(&mut self) -> SLC0_HOST_RD_ACK_INT_CLR_W {
        SLC0_HOST_RD_ACK_INT_CLR_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn slc0_wr_retry_done_int_clr(&mut self) -> SLC0_WR_RETRY_DONE_INT_CLR_W {
        SLC0_WR_RETRY_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slc0_tx_err_eof_int_clr(&mut self) -> SLC0_TX_ERR_EOF_INT_CLR_W {
        SLC0_TX_ERR_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cmd_dtc_int_clr(&mut self) -> CMD_DTC_INT_CLR_W {
        CMD_DTC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn slc0_rx_quick_eof_int_clr(&mut self) -> SLC0_RX_QUICK_EOF_INT_CLR_W {
        SLC0_RX_QUICK_EOF_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0int_clr](index.html) module"]
pub struct _0INT_CLR_SPEC;
impl crate::RegisterSpec for _0INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [_0int_clr::W](W) writer structure"]
impl crate::Writable for _0INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _0INT_CLR to value 0"]
impl crate::Resettable for _0INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
