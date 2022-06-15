#[doc = "Register `DMA_APB_I_2` reader"]
pub struct R(crate::R<DMA_APB_I_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_APB_I_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_APB_I_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_APB_I_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_APB_I_2` writer"]
pub struct W(crate::W<DMA_APB_I_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_APB_I_2_SPEC>;
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
impl From<crate::W<DMA_APB_I_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_APB_I_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_APB_I_ILG_CLR` reader - The clear signal for internal DMA access interrupt."]
pub type DMA_APB_I_ILG_CLR_R = crate::BitReader<bool>;
#[doc = "Field `DMA_APB_I_ILG_CLR` writer - The clear signal for internal DMA access interrupt."]
pub type DMA_APB_I_ILG_CLR_W<'a> = crate::BitWriter<'a, u32, DMA_APB_I_2_SPEC, bool, 0>;
#[doc = "Field `DMA_APB_I_ILG_EN` reader - The enable signal for internal DMA access interrupt."]
pub type DMA_APB_I_ILG_EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA_APB_I_ILG_EN` writer - The enable signal for internal DMA access interrupt."]
pub type DMA_APB_I_ILG_EN_W<'a> = crate::BitWriter<'a, u32, DMA_APB_I_2_SPEC, bool, 1>;
#[doc = "Field `DMA_APB_I_ILG_INTR` reader - Internal DMA access interrupt signal."]
pub type DMA_APB_I_ILG_INTR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The clear signal for internal DMA access interrupt."]
    #[inline(always)]
    pub fn dma_apb_i_ilg_clr(&self) -> DMA_APB_I_ILG_CLR_R {
        DMA_APB_I_ILG_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The enable signal for internal DMA access interrupt."]
    #[inline(always)]
    pub fn dma_apb_i_ilg_en(&self) -> DMA_APB_I_ILG_EN_R {
        DMA_APB_I_ILG_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Internal DMA access interrupt signal."]
    #[inline(always)]
    pub fn dma_apb_i_ilg_intr(&self) -> DMA_APB_I_ILG_INTR_R {
        DMA_APB_I_ILG_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The clear signal for internal DMA access interrupt."]
    #[inline(always)]
    pub fn dma_apb_i_ilg_clr(&mut self) -> DMA_APB_I_ILG_CLR_W {
        DMA_APB_I_ILG_CLR_W::new(self)
    }
    #[doc = "Bit 1 - The enable signal for internal DMA access interrupt."]
    #[inline(always)]
    pub fn dma_apb_i_ilg_en(&mut self) -> DMA_APB_I_ILG_EN_W {
        DMA_APB_I_ILG_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal DMA permission control register 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_apb_i_2](index.html) module"]
pub struct DMA_APB_I_2_SPEC;
impl crate::RegisterSpec for DMA_APB_I_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_apb_i_2::R](R) reader structure"]
impl crate::Readable for DMA_APB_I_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_apb_i_2::W](W) writer structure"]
impl crate::Writable for DMA_APB_I_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_APB_I_2 to value 0"]
impl crate::Resettable for DMA_APB_I_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
