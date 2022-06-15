#[doc = "Register `DMA_INT_SET` writer"]
pub struct W(crate::W<DMA_INT_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_INT_SET_SPEC>;
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
impl From<crate::W<DMA_INT_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_INT_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_INFIFO_FULL_ERR_INT_SET` writer - The software set bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
pub type DMA_INFIFO_FULL_ERR_INT_SET_W<'a> = crate::BitWriter<'a, u32, DMA_INT_SET_SPEC, bool, 0>;
#[doc = "Field `DMA_OUTFIFO_EMPTY_ERR_INT_SET` writer - The software set bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
pub type DMA_OUTFIFO_EMPTY_ERR_INT_SET_W<'a> = crate::BitWriter<'a, u32, DMA_INT_SET_SPEC, bool, 1>;
#[doc = "Field `SLV_EX_QPI_INT_SET` writer - The software set bit for SPI slave Ex_QPI interrupt."]
pub type SLV_EX_QPI_INT_SET_W<'a> = crate::BitWriter<'a, u32, DMA_INT_SET_SPEC, bool, 2>;
#[doc = "Field `SLV_EN_QPI_INT_SET` writer - The software set bit for SPI slave En_QPI interrupt."]
pub type SLV_EN_QPI_INT_SET_W<'a> = crate::BitWriter<'a, u32, DMA_INT_SET_SPEC, bool, 3>;
#[doc = "Field `SLV_CMD7_INT_SET` writer - The software set bit for SPI slave CMD7 interrupt."]
pub type SLV_CMD7_INT_SET_W<'a> = crate::BitWriter<'a, u32, DMA_INT_SET_SPEC, bool, 4>;
#[doc = "Field `SLV_CMD8_INT_SET` writer - The software set bit for SPI slave CMD8 interrupt."]
pub type SLV_CMD8_INT_SET_W<'a> = crate::BitWriter<'a, u32, DMA_INT_SET_SPEC, bool, 5>;
#[doc = "Field `SLV_CMD9_INT_SET` writer - The software set bit for SPI slave CMD9 interrupt."]
pub type SLV_CMD9_INT_SET_W<'a> = crate::BitWriter<'a, u32, DMA_INT_SET_SPEC, bool, 6>;
#[doc = "Field `SLV_CMDA_INT_SET` writer - The software set bit for SPI slave CMDA interrupt."]
pub type SLV_CMDA_INT_SET_W<'a> = crate::BitWriter<'a, u32, DMA_INT_SET_SPEC, bool, 7>;
#[doc = "Field `SLV_RD_DMA_DONE_INT_SET` writer - The software set bit for SPI_SLV_RD_DMA_DONE_INT interrupt."]
pub type SLV_RD_DMA_DONE_INT_SET_W<'a> = crate::BitWriter<'a, u32, DMA_INT_SET_SPEC, bool, 8>;
#[doc = "Field `SLV_WR_DMA_DONE_INT_SET` writer - The software set bit for SPI_SLV_WR_DMA_DONE_INT interrupt."]
pub type SLV_WR_DMA_DONE_INT_SET_W<'a> = crate::BitWriter<'a, u32, DMA_INT_SET_SPEC, bool, 9>;
#[doc = "Field `SLV_RD_BUF_DONE_INT_SET` writer - The software set bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
pub type SLV_RD_BUF_DONE_INT_SET_W<'a> = crate::BitWriter<'a, u32, DMA_INT_SET_SPEC, bool, 10>;
#[doc = "Field `SLV_WR_BUF_DONE_INT_SET` writer - The software set bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
pub type SLV_WR_BUF_DONE_INT_SET_W<'a> = crate::BitWriter<'a, u32, DMA_INT_SET_SPEC, bool, 11>;
#[doc = "Field `TRANS_DONE_INT_SET` writer - The software set bit for SPI_TRANS_DONE_INT interrupt."]
pub type TRANS_DONE_INT_SET_W<'a> = crate::BitWriter<'a, u32, DMA_INT_SET_SPEC, bool, 12>;
#[doc = "Field `DMA_SEG_TRANS_DONE_INT_SET` writer - The software set bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
pub type DMA_SEG_TRANS_DONE_INT_SET_W<'a> = crate::BitWriter<'a, u32, DMA_INT_SET_SPEC, bool, 13>;
#[doc = "Field `SEG_MAGIC_ERR_INT_SET` writer - The software set bit for SPI_SEG_MAGIC_ERR_INT interrupt."]
pub type SEG_MAGIC_ERR_INT_SET_W<'a> = crate::BitWriter<'a, u32, DMA_INT_SET_SPEC, bool, 14>;
#[doc = "Field `SLV_BUF_ADDR_ERR_INT_SET` writer - The software set bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
pub type SLV_BUF_ADDR_ERR_INT_SET_W<'a> = crate::BitWriter<'a, u32, DMA_INT_SET_SPEC, bool, 15>;
#[doc = "Field `SLV_CMD_ERR_INT_SET` writer - The software set bit for SPI_SLV_CMD_ERR_INT interrupt."]
pub type SLV_CMD_ERR_INT_SET_W<'a> = crate::BitWriter<'a, u32, DMA_INT_SET_SPEC, bool, 16>;
#[doc = "Field `MST_RX_AFIFO_WFULL_ERR_INT_SET` writer - The software set bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
pub type MST_RX_AFIFO_WFULL_ERR_INT_SET_W<'a> =
    crate::BitWriter<'a, u32, DMA_INT_SET_SPEC, bool, 17>;
#[doc = "Field `MST_TX_AFIFO_REMPTY_ERR_INT_SET` writer - The software set bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
pub type MST_TX_AFIFO_REMPTY_ERR_INT_SET_W<'a> =
    crate::BitWriter<'a, u32, DMA_INT_SET_SPEC, bool, 18>;
#[doc = "Field `APP2_INT_SET` writer - The software set bit for SPI_APP2_INT interrupt."]
pub type APP2_INT_SET_W<'a> = crate::BitWriter<'a, u32, DMA_INT_SET_SPEC, bool, 19>;
#[doc = "Field `APP1_INT_SET` writer - The software set bit for SPI_APP1_INT interrupt."]
pub type APP1_INT_SET_W<'a> = crate::BitWriter<'a, u32, DMA_INT_SET_SPEC, bool, 20>;
impl W {
    #[doc = "Bit 0 - The software set bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn dma_infifo_full_err_int_set(&mut self) -> DMA_INFIFO_FULL_ERR_INT_SET_W {
        DMA_INFIFO_FULL_ERR_INT_SET_W::new(self)
    }
    #[doc = "Bit 1 - The software set bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn dma_outfifo_empty_err_int_set(&mut self) -> DMA_OUTFIFO_EMPTY_ERR_INT_SET_W {
        DMA_OUTFIFO_EMPTY_ERR_INT_SET_W::new(self)
    }
    #[doc = "Bit 2 - The software set bit for SPI slave Ex_QPI interrupt."]
    #[inline(always)]
    pub fn slv_ex_qpi_int_set(&mut self) -> SLV_EX_QPI_INT_SET_W {
        SLV_EX_QPI_INT_SET_W::new(self)
    }
    #[doc = "Bit 3 - The software set bit for SPI slave En_QPI interrupt."]
    #[inline(always)]
    pub fn slv_en_qpi_int_set(&mut self) -> SLV_EN_QPI_INT_SET_W {
        SLV_EN_QPI_INT_SET_W::new(self)
    }
    #[doc = "Bit 4 - The software set bit for SPI slave CMD7 interrupt."]
    #[inline(always)]
    pub fn slv_cmd7_int_set(&mut self) -> SLV_CMD7_INT_SET_W {
        SLV_CMD7_INT_SET_W::new(self)
    }
    #[doc = "Bit 5 - The software set bit for SPI slave CMD8 interrupt."]
    #[inline(always)]
    pub fn slv_cmd8_int_set(&mut self) -> SLV_CMD8_INT_SET_W {
        SLV_CMD8_INT_SET_W::new(self)
    }
    #[doc = "Bit 6 - The software set bit for SPI slave CMD9 interrupt."]
    #[inline(always)]
    pub fn slv_cmd9_int_set(&mut self) -> SLV_CMD9_INT_SET_W {
        SLV_CMD9_INT_SET_W::new(self)
    }
    #[doc = "Bit 7 - The software set bit for SPI slave CMDA interrupt."]
    #[inline(always)]
    pub fn slv_cmda_int_set(&mut self) -> SLV_CMDA_INT_SET_W {
        SLV_CMDA_INT_SET_W::new(self)
    }
    #[doc = "Bit 8 - The software set bit for SPI_SLV_RD_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_rd_dma_done_int_set(&mut self) -> SLV_RD_DMA_DONE_INT_SET_W {
        SLV_RD_DMA_DONE_INT_SET_W::new(self)
    }
    #[doc = "Bit 9 - The software set bit for SPI_SLV_WR_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_wr_dma_done_int_set(&mut self) -> SLV_WR_DMA_DONE_INT_SET_W {
        SLV_WR_DMA_DONE_INT_SET_W::new(self)
    }
    #[doc = "Bit 10 - The software set bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_rd_buf_done_int_set(&mut self) -> SLV_RD_BUF_DONE_INT_SET_W {
        SLV_RD_BUF_DONE_INT_SET_W::new(self)
    }
    #[doc = "Bit 11 - The software set bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_wr_buf_done_int_set(&mut self) -> SLV_WR_BUF_DONE_INT_SET_W {
        SLV_WR_BUF_DONE_INT_SET_W::new(self)
    }
    #[doc = "Bit 12 - The software set bit for SPI_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn trans_done_int_set(&mut self) -> TRANS_DONE_INT_SET_W {
        TRANS_DONE_INT_SET_W::new(self)
    }
    #[doc = "Bit 13 - The software set bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn dma_seg_trans_done_int_set(&mut self) -> DMA_SEG_TRANS_DONE_INT_SET_W {
        DMA_SEG_TRANS_DONE_INT_SET_W::new(self)
    }
    #[doc = "Bit 14 - The software set bit for SPI_SEG_MAGIC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn seg_magic_err_int_set(&mut self) -> SEG_MAGIC_ERR_INT_SET_W {
        SEG_MAGIC_ERR_INT_SET_W::new(self)
    }
    #[doc = "Bit 15 - The software set bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn slv_buf_addr_err_int_set(&mut self) -> SLV_BUF_ADDR_ERR_INT_SET_W {
        SLV_BUF_ADDR_ERR_INT_SET_W::new(self)
    }
    #[doc = "Bit 16 - The software set bit for SPI_SLV_CMD_ERR_INT interrupt."]
    #[inline(always)]
    pub fn slv_cmd_err_int_set(&mut self) -> SLV_CMD_ERR_INT_SET_W {
        SLV_CMD_ERR_INT_SET_W::new(self)
    }
    #[doc = "Bit 17 - The software set bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn mst_rx_afifo_wfull_err_int_set(&mut self) -> MST_RX_AFIFO_WFULL_ERR_INT_SET_W {
        MST_RX_AFIFO_WFULL_ERR_INT_SET_W::new(self)
    }
    #[doc = "Bit 18 - The software set bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn mst_tx_afifo_rempty_err_int_set(&mut self) -> MST_TX_AFIFO_REMPTY_ERR_INT_SET_W {
        MST_TX_AFIFO_REMPTY_ERR_INT_SET_W::new(self)
    }
    #[doc = "Bit 19 - The software set bit for SPI_APP2_INT interrupt."]
    #[inline(always)]
    pub fn app2_int_set(&mut self) -> APP2_INT_SET_W {
        APP2_INT_SET_W::new(self)
    }
    #[doc = "Bit 20 - The software set bit for SPI_APP1_INT interrupt."]
    #[inline(always)]
    pub fn app1_int_set(&mut self) -> APP1_INT_SET_W {
        APP1_INT_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI interrupt software set register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_set](index.html) module"]
pub struct DMA_INT_SET_SPEC;
impl crate::RegisterSpec for DMA_INT_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dma_int_set::W](W) writer structure"]
impl crate::Writable for DMA_INT_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_INT_SET to value 0"]
impl crate::Resettable for DMA_INT_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
