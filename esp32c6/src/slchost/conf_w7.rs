#[doc = "Register `CONF_W7` reader"]
pub type R = crate::R<CONF_W7_SPEC>;
#[doc = "Register `CONF_W7` writer"]
pub type W = crate::W<CONF_W7_SPEC>;
#[doc = "Field `SLCHOST_CONF28` reader - *******Description***********"]
pub type SLCHOST_CONF28_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF28` writer - *******Description***********"]
pub type SLCHOST_CONF28_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SLCHOST_CONF29` reader - *******Description***********"]
pub type SLCHOST_CONF29_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF29` writer - *******Description***********"]
pub type SLCHOST_CONF29_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SLCHOST_CONF30` reader - *******Description***********"]
pub type SLCHOST_CONF30_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF30` writer - *******Description***********"]
pub type SLCHOST_CONF30_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SLCHOST_CONF31` reader - *******Description***********"]
pub type SLCHOST_CONF31_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF31` writer - *******Description***********"]
pub type SLCHOST_CONF31_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf28(&self) -> SLCHOST_CONF28_R {
        SLCHOST_CONF28_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf29(&self) -> SLCHOST_CONF29_R {
        SLCHOST_CONF29_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf30(&self) -> SLCHOST_CONF30_R {
        SLCHOST_CONF30_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf31(&self) -> SLCHOST_CONF31_R {
        SLCHOST_CONF31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W7")
            .field(
                "slchost_conf28",
                &format_args!("{}", self.slchost_conf28().bits()),
            )
            .field(
                "slchost_conf29",
                &format_args!("{}", self.slchost_conf29().bits()),
            )
            .field(
                "slchost_conf30",
                &format_args!("{}", self.slchost_conf30().bits()),
            )
            .field(
                "slchost_conf31",
                &format_args!("{}", self.slchost_conf31().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf28(&mut self) -> SLCHOST_CONF28_W<CONF_W7_SPEC, 0> {
        SLCHOST_CONF28_W::new(self)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf29(&mut self) -> SLCHOST_CONF29_W<CONF_W7_SPEC, 8> {
        SLCHOST_CONF29_W::new(self)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf30(&mut self) -> SLCHOST_CONF30_W<CONF_W7_SPEC, 16> {
        SLCHOST_CONF30_W::new(self)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf31(&mut self) -> SLCHOST_CONF31_W<CONF_W7_SPEC, 24> {
        SLCHOST_CONF31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf_w7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf_w7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_W7_SPEC;
impl crate::RegisterSpec for CONF_W7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf_w7::R`](R) reader structure"]
impl crate::Readable for CONF_W7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf_w7::W`](W) writer structure"]
impl crate::Writable for CONF_W7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF_W7 to value 0"]
impl crate::Resettable for CONF_W7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
