#[doc = "Register `ESC_CONF1` reader"]
pub type R = crate::R<ESC_CONF1_SPEC>;
#[doc = "Register `ESC_CONF1` writer"]
pub type W = crate::W<ESC_CONF1_SPEC>;
#[doc = "Field `ESC_SEQ0` reader - This register stores the first substitute char used to replace the seperator char."]
pub type ESC_SEQ0_R = crate::FieldReader;
#[doc = "Field `ESC_SEQ0` writer - This register stores the first substitute char used to replace the seperator char."]
pub type ESC_SEQ0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `ESC_SEQ0_CHAR0` reader - This register stores the first char used to replace reg_esc_seq0 in data."]
pub type ESC_SEQ0_CHAR0_R = crate::FieldReader;
#[doc = "Field `ESC_SEQ0_CHAR0` writer - This register stores the first char used to replace reg_esc_seq0 in data."]
pub type ESC_SEQ0_CHAR0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `ESC_SEQ0_CHAR1` reader - This register stores the second char used to replace the reg_esc_seq0 in data"]
pub type ESC_SEQ0_CHAR1_R = crate::FieldReader;
#[doc = "Field `ESC_SEQ0_CHAR1` writer - This register stores the second char used to replace the reg_esc_seq0 in data"]
pub type ESC_SEQ0_CHAR1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - This register stores the first substitute char used to replace the seperator char."]
    #[inline(always)]
    pub fn esc_seq0(&self) -> ESC_SEQ0_R {
        ESC_SEQ0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This register stores the first char used to replace reg_esc_seq0 in data."]
    #[inline(always)]
    pub fn esc_seq0_char0(&self) -> ESC_SEQ0_CHAR0_R {
        ESC_SEQ0_CHAR0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This register stores the second char used to replace the reg_esc_seq0 in data"]
    #[inline(always)]
    pub fn esc_seq0_char1(&self) -> ESC_SEQ0_CHAR1_R {
        ESC_SEQ0_CHAR1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESC_CONF1")
            .field("esc_seq0", &format_args!("{}", self.esc_seq0().bits()))
            .field(
                "esc_seq0_char0",
                &format_args!("{}", self.esc_seq0_char0().bits()),
            )
            .field(
                "esc_seq0_char1",
                &format_args!("{}", self.esc_seq0_char1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ESC_CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register stores the first substitute char used to replace the seperator char."]
    #[inline(always)]
    #[must_use]
    pub fn esc_seq0(&mut self) -> ESC_SEQ0_W<ESC_CONF1_SPEC, 0> {
        ESC_SEQ0_W::new(self)
    }
    #[doc = "Bits 8:15 - This register stores the first char used to replace reg_esc_seq0 in data."]
    #[inline(always)]
    #[must_use]
    pub fn esc_seq0_char0(&mut self) -> ESC_SEQ0_CHAR0_W<ESC_CONF1_SPEC, 8> {
        ESC_SEQ0_CHAR0_W::new(self)
    }
    #[doc = "Bits 16:23 - This register stores the second char used to replace the reg_esc_seq0 in data"]
    #[inline(always)]
    #[must_use]
    pub fn esc_seq0_char1(&mut self) -> ESC_SEQ0_CHAR1_W<ESC_CONF1_SPEC, 16> {
        ESC_SEQ0_CHAR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esc_conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`esc_conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ESC_CONF1_SPEC;
impl crate::RegisterSpec for ESC_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esc_conf1::R`](R) reader structure"]
impl crate::Readable for ESC_CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`esc_conf1::W`](W) writer structure"]
impl crate::Writable for ESC_CONF1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ESC_CONF1 to value 0x00dd_dbdb"]
impl crate::Resettable for ESC_CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0x00dd_dbdb;
}
