#[doc = "Register `CARDTHRCTL` reader"]
pub type R = crate::R<CARDTHRCTL_SPEC>;
#[doc = "Register `CARDTHRCTL` writer"]
pub type W = crate::W<CARDTHRCTL_SPEC>;
#[doc = "Field `CARDRDTHREN` reader - Card read threshold enable. 1'b0-Card read threshold disabled. 1'b1-Card read threshold enabled."]
pub type CARDRDTHREN_R = crate::BitReader;
#[doc = "Field `CARDRDTHREN` writer - Card read threshold enable. 1'b0-Card read threshold disabled. 1'b1-Card read threshold enabled."]
pub type CARDRDTHREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CARDCLRINTEN` reader - Busy clear interrupt generation: 1'b0-Busy clear interrypt disabled. 1'b1-Busy clear interrypt enabled."]
pub type CARDCLRINTEN_R = crate::BitReader;
#[doc = "Field `CARDCLRINTEN` writer - Busy clear interrupt generation: 1'b0-Busy clear interrypt disabled. 1'b1-Busy clear interrypt enabled."]
pub type CARDCLRINTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CARDWRTHREN` reader - Applicable when HS400 mode is enabled. 1'b0-Card write Threshold disabled. 1'b1-Card write Threshold enabled."]
pub type CARDWRTHREN_R = crate::BitReader;
#[doc = "Field `CARDWRTHREN` writer - Applicable when HS400 mode is enabled. 1'b0-Card write Threshold disabled. 1'b1-Card write Threshold enabled."]
pub type CARDWRTHREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CARDTHRESHOLD` reader - The inside FIFO size is 512,This register is applicable when SDHOST_CARDERTHREN_REG is set to 1 or SDHOST_CARDRDTHREN_REG set to 1."]
pub type CARDTHRESHOLD_R = crate::FieldReader<u16>;
#[doc = "Field `CARDTHRESHOLD` writer - The inside FIFO size is 512,This register is applicable when SDHOST_CARDERTHREN_REG is set to 1 or SDHOST_CARDRDTHREN_REG set to 1."]
pub type CARDTHRESHOLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - Card read threshold enable. 1'b0-Card read threshold disabled. 1'b1-Card read threshold enabled."]
    #[inline(always)]
    pub fn cardrdthren(&self) -> CARDRDTHREN_R {
        CARDRDTHREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Busy clear interrupt generation: 1'b0-Busy clear interrypt disabled. 1'b1-Busy clear interrypt enabled."]
    #[inline(always)]
    pub fn cardclrinten(&self) -> CARDCLRINTEN_R {
        CARDCLRINTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Applicable when HS400 mode is enabled. 1'b0-Card write Threshold disabled. 1'b1-Card write Threshold enabled."]
    #[inline(always)]
    pub fn cardwrthren(&self) -> CARDWRTHREN_R {
        CARDWRTHREN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 16:31 - The inside FIFO size is 512,This register is applicable when SDHOST_CARDERTHREN_REG is set to 1 or SDHOST_CARDRDTHREN_REG set to 1."]
    #[inline(always)]
    pub fn cardthreshold(&self) -> CARDTHRESHOLD_R {
        CARDTHRESHOLD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CARDTHRCTL")
            .field("cardrdthren", &format_args!("{}", self.cardrdthren().bit()))
            .field(
                "cardclrinten",
                &format_args!("{}", self.cardclrinten().bit()),
            )
            .field("cardwrthren", &format_args!("{}", self.cardwrthren().bit()))
            .field(
                "cardthreshold",
                &format_args!("{}", self.cardthreshold().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CARDTHRCTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Card read threshold enable. 1'b0-Card read threshold disabled. 1'b1-Card read threshold enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cardrdthren(&mut self) -> CARDRDTHREN_W<CARDTHRCTL_SPEC, 0> {
        CARDRDTHREN_W::new(self)
    }
    #[doc = "Bit 1 - Busy clear interrupt generation: 1'b0-Busy clear interrypt disabled. 1'b1-Busy clear interrypt enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cardclrinten(&mut self) -> CARDCLRINTEN_W<CARDTHRCTL_SPEC, 1> {
        CARDCLRINTEN_W::new(self)
    }
    #[doc = "Bit 2 - Applicable when HS400 mode is enabled. 1'b0-Card write Threshold disabled. 1'b1-Card write Threshold enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cardwrthren(&mut self) -> CARDWRTHREN_W<CARDTHRCTL_SPEC, 2> {
        CARDWRTHREN_W::new(self)
    }
    #[doc = "Bits 16:31 - The inside FIFO size is 512,This register is applicable when SDHOST_CARDERTHREN_REG is set to 1 or SDHOST_CARDRDTHREN_REG set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn cardthreshold(&mut self) -> CARDTHRESHOLD_W<CARDTHRCTL_SPEC, 16> {
        CARDTHRESHOLD_W::new(self)
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
#[doc = "Card Threshold Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cardthrctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cardthrctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CARDTHRCTL_SPEC;
impl crate::RegisterSpec for CARDTHRCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cardthrctl::R`](R) reader structure"]
impl crate::Readable for CARDTHRCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cardthrctl::W`](W) writer structure"]
impl crate::Writable for CARDTHRCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CARDTHRCTL to value 0"]
impl crate::Resettable for CARDTHRCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
