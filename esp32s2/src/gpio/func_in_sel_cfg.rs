#[doc = "Register `FUNC%s_IN_SEL_CFG` reader"]
pub type R = crate::R<FUNC_IN_SEL_CFG_SPEC>;
#[doc = "Register `FUNC%s_IN_SEL_CFG` writer"]
pub type W = crate::W<FUNC_IN_SEL_CFG_SPEC>;
#[doc = "Field `IN_SEL` reader - Selection control for peripheral input signal m, selects a pad from the 54 GPIO matrix pads to connect this input signal. Or selects 0x38 for a constantly high input or 0x3C for a constantly low input."]
pub type IN_SEL_R = crate::FieldReader;
#[doc = "Field `IN_SEL` writer - Selection control for peripheral input signal m, selects a pad from the 54 GPIO matrix pads to connect this input signal. Or selects 0x38 for a constantly high input or 0x3C for a constantly low input."]
pub type IN_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `IN_INV_SEL` reader - Invert the input value. 1: invert enabled; 0: invert disabled."]
pub type IN_INV_SEL_R = crate::BitReader;
#[doc = "Field `IN_INV_SEL` writer - Invert the input value. 1: invert enabled; 0: invert disabled."]
pub type IN_INV_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SEL` reader - Bypass GPIO matrix. 1: route signals via GPIO matrix, 0: connect signals directly to peripheral configured in IO_MUX."]
pub type SEL_R = crate::BitReader;
#[doc = "Field `SEL` writer - Bypass GPIO matrix. 1: route signals via GPIO matrix, 0: connect signals directly to peripheral configured in IO_MUX."]
pub type SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:5 - Selection control for peripheral input signal m, selects a pad from the 54 GPIO matrix pads to connect this input signal. Or selects 0x38 for a constantly high input or 0x3C for a constantly low input."]
    #[inline(always)]
    pub fn in_sel(&self) -> IN_SEL_R {
        IN_SEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Invert the input value. 1: invert enabled; 0: invert disabled."]
    #[inline(always)]
    pub fn in_inv_sel(&self) -> IN_INV_SEL_R {
        IN_INV_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bypass GPIO matrix. 1: route signals via GPIO matrix, 0: connect signals directly to peripheral configured in IO_MUX."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC_IN_SEL_CFG")
            .field("in_sel", &format_args!("{}", self.in_sel().bits()))
            .field("in_inv_sel", &format_args!("{}", self.in_inv_sel().bit()))
            .field("sel", &format_args!("{}", self.sel().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FUNC_IN_SEL_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Selection control for peripheral input signal m, selects a pad from the 54 GPIO matrix pads to connect this input signal. Or selects 0x38 for a constantly high input or 0x3C for a constantly low input."]
    #[inline(always)]
    #[must_use]
    pub fn in_sel(&mut self) -> IN_SEL_W<FUNC_IN_SEL_CFG_SPEC, 0> {
        IN_SEL_W::new(self)
    }
    #[doc = "Bit 6 - Invert the input value. 1: invert enabled; 0: invert disabled."]
    #[inline(always)]
    #[must_use]
    pub fn in_inv_sel(&mut self) -> IN_INV_SEL_W<FUNC_IN_SEL_CFG_SPEC, 6> {
        IN_INV_SEL_W::new(self)
    }
    #[doc = "Bit 7 - Bypass GPIO matrix. 1: route signals via GPIO matrix, 0: connect signals directly to peripheral configured in IO_MUX."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<FUNC_IN_SEL_CFG_SPEC, 7> {
        SEL_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Peripheral function %s input selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_in_sel_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_in_sel_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FUNC_IN_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC_IN_SEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func_in_sel_cfg::R`](R) reader structure"]
impl crate::Readable for FUNC_IN_SEL_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`func_in_sel_cfg::W`](W) writer structure"]
impl crate::Writable for FUNC_IN_SEL_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FUNC%s_IN_SEL_CFG to value 0"]
impl crate::Resettable for FUNC_IN_SEL_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
