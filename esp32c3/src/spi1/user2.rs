#[doc = "Register `USER2` reader"]
pub struct R(crate::R<USER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USER2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USER2` writer"]
pub struct W(crate::W<USER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USER2_SPEC>;
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
impl From<crate::W<USER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USER2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USR_COMMAND_VALUE` reader - The value of command."]
pub type USR_COMMAND_VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `USR_COMMAND_VALUE` writer - The value of command."]
pub type USR_COMMAND_VALUE_W<'a> = crate::FieldWriter<'a, u32, USER2_SPEC, u16, u16, 16, 0>;
#[doc = "Field `USR_COMMAND_BITLEN` reader - The length in bits of command phase. The register value shall be (bit_num-1)"]
pub type USR_COMMAND_BITLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USR_COMMAND_BITLEN` writer - The length in bits of command phase. The register value shall be (bit_num-1)"]
pub type USR_COMMAND_BITLEN_W<'a> = crate::FieldWriter<'a, u32, USER2_SPEC, u8, u8, 4, 28>;
impl R {
    #[doc = "Bits 0:15 - The value of command."]
    #[inline(always)]
    pub fn usr_command_value(&self) -> USR_COMMAND_VALUE_R {
        USR_COMMAND_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - The length in bits of command phase. The register value shall be (bit_num-1)"]
    #[inline(always)]
    pub fn usr_command_bitlen(&self) -> USR_COMMAND_BITLEN_R {
        USR_COMMAND_BITLEN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - The value of command."]
    #[inline(always)]
    pub fn usr_command_value(&mut self) -> USR_COMMAND_VALUE_W {
        USR_COMMAND_VALUE_W::new(self)
    }
    #[doc = "Bits 28:31 - The length in bits of command phase. The register value shall be (bit_num-1)"]
    #[inline(always)]
    pub fn usr_command_bitlen(&mut self) -> USR_COMMAND_BITLEN_W {
        USR_COMMAND_BITLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 user2 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user2](index.html) module"]
pub struct USER2_SPEC;
impl crate::RegisterSpec for USER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [user2::R](R) reader structure"]
impl crate::Readable for USER2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [user2::W](W) writer structure"]
impl crate::Writable for USER2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USER2 to value 0x7000_0000"]
impl crate::Resettable for USER2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7000_0000
    }
}
